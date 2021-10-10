use std::ops::{ControlFlow, FromResidual, Try};

use super::structure::{Environment, OpaqueId, Substitutions, Value, ValueId};
use crate::shared::{OpaqueClass, OrderedMap};

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
                self.reduce(id);
                match self.values.next(id) {
                    Some(next) => id = next,
                    None => break,
                }
            }
        }
    }

    pub fn reduce(&mut self, id: ValueId) -> ValueId {
        let mut reducer = Reducer {
            env: self,
            stack: Vec::new(),
        };
        let reduced = reducer.reduce(id);
        debug_assert_eq!(reducer.stack.len(), 0);
        reduced
    }

    fn split_into_base_and_substitutions(
        &mut self,
        unsplit: ValueId,
    ) -> (ValueId, OrderedMap<OpaqueId, Option<ValueId>>) {
        match self.values[unsplit].value.clone() {
            Value::Opaque { .. } => {
                let deps = self.dependencies(unsplit);
                let deps_as_subs = deps.into_iter().map(|d| (d, None)).collect();
                (unsplit, deps_as_subs)
            }
            Value::Substituting {
                base,
                substitutions,
            } => {
                let (full_base, base_subs) = self.split_into_base_and_substitutions(base);
                let substitutions = substitutions
                    .into_iter()
                    .map(|s| (s.0, Some(s.1)))
                    .collect();
                (full_base, base_subs.union(substitutions))
            }
            _ => (unsplit, Default::default()),
        }
    }

    fn substitution_matches_condition(
        &mut self,
        base: ValueId,
        substitutions: Substitutions,
        condition: ValueId,
        vars_to_bind: &[OpaqueId],
    ) -> MatchResult {
        let (base_cond, cond_vars) = self.split_into_base_and_substitutions(condition);
        let mut result_subs = self.matches(base, base_cond, vars_to_bind)?;
        for (target, cond_value) in cond_vars {
            if let Some(&substitution) = substitutions.get(&target) {
                // If the base being matched replaces target with substitution...
                if let Some(cond_value) = cond_value {
                    // If the condition replaces target with cond_value...
                    // Then try to match substitution using cond_value as the condition.
                    let subs_here = self.matches(substitution, cond_value, vars_to_bind)?;
                    for (target, value) in subs_here {
                        if result_subs.contains_key(&target) {
                            todo!("Nice error, pattern contains same variable twice.");
                        }
                        result_subs.insert_no_replace(target, value);
                    }
                } else {
                    // If the condition is just a variable...
                    if result_subs.contains_key(&target) {
                        todo!("Nice error, pattern contains same variable twice.");
                    }
                    // Then just replace that variable with the base value.
                    result_subs.insert_no_replace(target, substitution);
                }
            } else if cond_value.is_some() {
                return MatchResult::Uncertain;
            } else {
                continue;
            }
        }
        MatchResult::Match { subs: result_subs }
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
                _,
            ) => self.substitution_matches_condition(base, substitutions, condition, vars_to_bind),
            _ => MatchResult::Uncertain,
        }
    }
}

struct Reducer<'a> {
    env: &'a mut Environment,
    stack: Vec<ValueId>,
}

impl<'a> Reducer<'a> {
    fn reduce_match(&mut self, base: ValueId, old_cases: Vec<(ValueId, ValueId)>) -> ValueId {
        let base = self.reduce(base);
        let mut cases = Vec::new();
        for (condition, value) in old_cases {
            let condition = self.reduce(condition);
            let condition_deps = self.env.dependencies(condition);
            match self.env.matches(base, condition, &condition_deps) {
                MatchResult::Match { subs } => {
                    if cases.len() == 0 {
                        let mut value = value;
                        value = self.env.substitute(value, &subs);
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
        self.env.gpv(Value::Match { base, cases })
    }

    fn reduce_from_scratch(&mut self, of: ValueId) -> ValueId {
        match self.env.values[of].value.clone() {
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(..) => of,
            Value::From { base, variable } => {
                let base = self.reduce(base);
                let value = Value::From { base, variable };
                self.env.gpv(value)
            }
            Value::Match { base, cases } => self.reduce_match(base, cases),
            Value::Opaque { class, id, typee } => {
                let typee = self.reduce(typee);
                let value = Value::Opaque { class, id, typee };
                self.env.gpv(value)
            }
            Value::Placeholder(..) => unreachable!(),
            Value::Substituting {
                base,
                substitutions,
            } => {
                let rsubs = substitutions
                    .into_iter()
                    .map(|(t, v)| (t, self.reduce(v)))
                    .collect();
                let subbed = self.env.substitute(base, &rsubs);
                if subbed == of {
                    subbed
                } else {
                    self.reduce(subbed)
                }
            }
        }
    }

    pub fn reduce(&mut self, of: ValueId) -> ValueId {
        if self.stack.contains(&of) {
            of
        } else if let Some(cached) = self.env.values[of].cached_reduction {
            cached
        } else {
            self.stack.push(of);
            let reduced = self.reduce_from_scratch(of);
            self.env.values[of].cached_reduction = Some(reduced);
            self.env.values[reduced].cached_reduction = Some(reduced);
            if self.env.values[reduced].defined_at.is_empty() {
                self.env.values[reduced].defined_at = self.env.values[of].defined_at.clone();
            }
            self.env.values[reduced].referenced_at = self.env.values[reduced]
                .referenced_at
                .clone()
                .union(self.env.values[of].referenced_at.clone());
            for (from, _) in self.env.values[of].display_requested_from.take() {
                self.env.values[reduced]
                    .display_requested_from
                    .insert_or_replace(from, ());
            }
            debug_assert_eq!(self.reduce(reduced), reduced);
            let typee = self.env.get_type(of);
            let rtype = self.env.get_type(reduced);
            if !self.env.type_is_base_of_other(rtype, typee) {
                println!("{:#?}", self.env);
                println!("{:?} was reduced to {:?}", of, reduced);
                println!("but the new type {:?}", rtype);
                println!("is not a base of the original type {:?}", typee);
                panic!();
            }
            self.stack.pop();
            reduced
        }
    }
}