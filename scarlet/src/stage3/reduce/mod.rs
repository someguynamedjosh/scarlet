use std::ops::{ControlFlow, FromResidual, Try};

use super::structure::{AnnotatedValue, Environment, OpaqueId, Substitutions, Value, ValueId};
use crate::shared::OpaqueClass;

#[derive(Clone, Debug)]
enum MatchResult {
    Match { subs: Substitutions },
    NoMatch,
    Uncertain,
}

impl FromResidual for MatchResult {
    fn from_residual(residual: Self) -> Self {
        residual
    }
}

impl Try for MatchResult {
    type Output = Substitutions;
    type Residual = Self;

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            MatchResult::Match { subs } => ControlFlow::Continue(subs),
            _ => ControlFlow::Break(self),
        }
    }

    fn from_output(output: Self::Output) -> Self {
        Self::Match { subs: output }
    }
}

impl From<bool> for MatchResult {
    fn from(cond: bool) -> Self {
        if cond {
            Self::Match {
                subs: Substitutions::new(),
            }
        } else {
            Self::NoMatch
        }
    }
}

impl From<Option<bool>> for MatchResult {
    fn from(cond: Option<bool>) -> Self {
        match cond {
            Some(cond) => cond.into(),
            None => Self::Uncertain,
        }
    }
}

impl Environment {
    pub fn reduce_all(&mut self) {
        if let Some(start) = self.values.first() {
            let mut id = start;
            loop {
                println!("{:#?}", self);
                self.reduce(id);
                match self.values.next(id) {
                    Some(next) => id = next,
                    None => break,
                }
            }
        }
    }

    fn is_var(&self, value: &AnnotatedValue, var: OpaqueId) -> bool {
        match &value.value {
            Value::Opaque { id, .. } => *id == var,
            _ => false,
        }
    }

    fn matches(
        &mut self,
        base: ValueId,
        condition: ValueId,
        vars_to_bind: &[OpaqueId],
    ) -> MatchResult {
        let base_val = self.values[base].value.clone();
        let condition_val = self.values[condition].value.clone();
        match (base_val, condition_val) {
            (
                _,
                Value::Opaque {
                    class: OpaqueClass::Variable,
                    id,
                    ..
                },
            ) => {
                if vars_to_bind.contains(&id) {
                    let subs = std::iter::once((id, base)).collect();
                    MatchResult::Match { subs }
                } else {
                    MatchResult::Uncertain
                }
            }
            (
                Value::Opaque {
                    class: OpaqueClass::Instance,
                    id: base_id,
                    ..
                },
                Value::Opaque {
                    class: OpaqueClass::Instance,
                    id: condition_id,
                    ..
                },
            ) => {
                if base_id == condition_id {
                    MatchResult::Match {
                        subs: Default::default(),
                    }
                } else {
                    MatchResult::NoMatch
                }
            }
            (
                Value::Substituting {
                    base,
                    substitutions,
                },
                Value::Opaque {
                    class: OpaqueClass::Instance,
                    typee,
                    ..
                },
            ) => {
                let mut result_subs = self.matches(base, condition, vars_to_bind)?;
                let type_vars = self.get_from_variables(typee);
                for (target, value) in substitutions {
                    if !type_vars.contains_key(&target) {
                        continue;
                    } else if result_subs.contains_key(&target) {
                        todo!("Nice error, cannot bind to the same variable multiple times.");
                    } else if vars_to_bind.contains(&target) {
                        result_subs.insert_no_replace(target, value);
                    } else {
                        return MatchResult::Uncertain;
                    }
                }
                todo!()
            }
            _ => MatchResult::Uncertain,
        }
    }

    fn reduce_match(&mut self, base: ValueId, old_cases: Vec<(ValueId, ValueId)>) -> ValueId {
        let base = self.reduce(base);
        let mut cases = Vec::new();
        for (condition, value) in old_cases {
            let condition = self.reduce(condition);
            let condition_deps = self.dependencies(condition);
            match self.matches(base, condition, &condition_deps) {
                MatchResult::Match { subs } => {
                    if cases.len() == 0 {
                        let mut value = value;
                        value = self.substitute(value, &subs);
                        return self.reduce(value);
                    } else {
                        let value = self.reduce(value);
                        cases.push((condition, value));
                        break;
                    }
                }
                MatchResult::NoMatch => (),
                MatchResult::Uncertain => {
                    let value = self.reduce(value);
                    cases.push((condition, value));
                }
            }
        }
        self.gpv(Value::Match { base, cases })
    }

    fn reduce_from_scratch(&mut self, of: ValueId) -> ValueId {
        match self.values[of].value.clone() {
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(..) => of,
            Value::From { base, variable } => {
                let base = self.reduce(base);
                let value = Value::From { base, variable };
                self.gpv(value)
            }
            Value::Match { base, cases } => self.reduce_match(base, cases),
            Value::Opaque { class, id, typee } => {
                let typee = self.reduce(typee);
                let value = Value::Opaque { class, id, typee };
                self.gpv(value)
            }
            Value::Substituting {
                base,
                substitutions,
            } => {
                println!("{:#?}", self);
                let subbed = self.substitute(base, &substitutions);
                self.reduce(subbed)
            }
        }
    }

    pub fn reduce(&mut self, of: ValueId) -> ValueId {
        if let Some(cached) = self.values[of].cached_reduction {
            cached
        } else {
            let reduced = self.reduce_from_scratch(of);
            self.values[of].cached_reduction = Some(reduced);
            self.values[reduced].cached_reduction = Some(reduced);
            if self.values[reduced].defined_at.is_empty() {
                self.values[reduced].defined_at = self.values[of].defined_at.clone();
            }
            self.values[reduced].referenced_at = self.values[reduced]
                .referenced_at
                .clone()
                .union(self.values[of].referenced_at.clone());
            for (from, _) in self.values[of].display_requested_from.take() {
                self.values[reduced]
                    .display_requested_from
                    .insert_or_replace(from, ());
            }
            debug_assert_eq!(self.reduce(reduced), reduced);
            let typee = self.get_type(of);
            let rtype = self.get_type(reduced);
            if !self.type_is_base_of_other(rtype, typee) {
                println!("{:#?}", self);
                println!("{:?} was reduced to {:?}", of, reduced);
                println!("but the new type {:?}", rtype);
                println!("is not a base of the original type {:?}", typee);
                panic!();
            }
            reduced
        }
    }
}
