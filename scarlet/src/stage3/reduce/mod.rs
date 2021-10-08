use super::structure::{AnnotatedValue, Environment, OpaqueId, Value, ValueId};
use crate::shared::OpaqueClass;

type Substitutions = Vec<(OpaqueId, ValueId)>;
type Targets = Vec<(OpaqueId, Target)>;

#[derive(Clone, Debug)]
enum MatchResult {
    Match { subs: Substitutions },
    NoMatch,
    Uncertain,
}

impl From<bool> for MatchResult {
    fn from(cond: bool) -> Self {
        if cond {
            Self::Match { subs: vec![] }
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

#[derive(Clone, Debug)]
enum Target {
    BoundVariable(OpaqueId),
    LiteralValue(ValueId),
    Variant { id: OpaqueId, values: Targets },
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

    fn condition_target(&mut self, condition: ValueId, targets_so_far: Targets) -> Target {
        match &self.values[condition].value {
            Value::Opaque { class, id, typee } => match class {
                OpaqueClass::Variable => Target::BoundVariable(*id),
                OpaqueClass::Variant => {
                    let deps = self.get_from_variables(*typee);
                    let mut values = targets_so_far;
                    for (dep, _) in deps {
                        if !values.iter().any(|(t, _)| *t == dep) {
                            values.push((dep, Target::BoundVariable(dep)));
                        }
                    }
                    let id = *id;
                    Target::Variant { id, values }
                }
            },
            &Value::Substituting {
                base,
                target,
                value,
            } => {
                let value = self.condition_target(value, vec![]);
                let mut targets_so_far = targets_so_far;
                targets_so_far.push((target, value));
                self.condition_target(base, targets_so_far)
            }
            _ => {
                if targets_so_far.len() == 0 {
                    Target::LiteralValue(condition)
                } else {
                    unreachable!("I think?")
                }
            }
        }
    }

    fn dereference_subs(&mut self, base: ValueId) -> (ValueId, Substitutions) {
        if let &Value::Substituting {
            base,
            target,
            value,
        } = &self.values[base].value
        {
            let (base, mut subs) = self.dereference_subs(base);
            subs.push((target, value));
            (base, subs)
        } else {
            (base, vec![])
        }
    }

    fn subs_are_def_equal(&mut self, a: &Substitutions, b: &Substitutions) -> Option<bool> {
        todo!()
    }

    fn are_def_equal(&mut self, a: ValueId, b: ValueId) -> Option<bool> {
        let (a, a_subs) = self.dereference_subs(a);
        let (b, b_subs) = self.dereference_subs(b);
        match (&self.values[a].value, &self.values[b].value) {
            (Value::BuiltinOperation(..), Value::BuiltinOperation(..)) => todo!(),
            (Value::BuiltinValue(a), Value::BuiltinValue(b)) => Some(a == b),
            (&Value::Opaque { id: aid, .. }, &Value::Opaque { id: bid, .. }) => {
                Some(aid == bid && self.subs_are_def_equal(&a_subs, &b_subs)?)
            }
            _ => None,
        }
    }

    fn is_var(&self, value: &AnnotatedValue, var: OpaqueId) -> bool {
        match &value.value {
            Value::Opaque { id, .. } => *id == var,
            _ => false,
        }
    }

    fn matches_target(&mut self, base: ValueId, target: Target) -> MatchResult {
        match target {
            Target::BoundVariable(var) => {
                let subs = vec![(var, base)];
                MatchResult::Match { subs }
            }
            Target::LiteralValue(val) => self.are_def_equal(base, val).into(),
            Target::Variant { id, values } => {
                let (base, mut base_subs) = self.dereference_subs(base);
                let typee = match &self.values[base].value {
                    Value::Opaque {
                        class: OpaqueClass::Variant,
                        id: base_id,
                        typee,
                    } => {
                        if *base_id != id {
                            return MatchResult::NoMatch;
                        } else {
                            *typee
                        }
                    }
                    _ => return MatchResult::Uncertain,
                };
                for (dep, _) in self.get_from_variables(typee) {
                    if !base_subs.iter().any(|x| x.0 == dep) {
                        let (var, _) = self.values.iter().find(|v| self.is_var(v.1, dep)).unwrap();
                        base_subs.push((dep, var));
                    }
                }
                // Not sure yet when this would be false.
                assert_eq!(values.len(), base_subs.len());
                let mut total_subs = Vec::new();
                for (var, target) in values {
                    let index = base_subs.iter().position(|x| x.0 == var).unwrap();
                    let value = base_subs.remove(index).1;
                    match self.matches_target(value, target) {
                        MatchResult::Match { mut subs } => total_subs.append(&mut subs),
                        MatchResult::NoMatch => return MatchResult::NoMatch,
                        MatchResult::Uncertain => return MatchResult::Uncertain,
                    }
                }
                MatchResult::Match { subs: total_subs }
            }
        }
    }

    fn matches(&mut self, base: ValueId, condition: ValueId) -> MatchResult {
        let condition_target = self.condition_target(condition, vec![]);
        let result = self.matches_target(base, condition_target.clone());
        result
    }

    fn reduce_from_scratch(&mut self, of: ValueId) -> ValueId {
        match &self.values[of].value {
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(..) => of,
            Value::From { base, variable } => {
                let (base, variable) = (*base, *variable);
                let base = self.reduce(base);
                let value = Value::From { base, variable };
                self.gpv(value)
            }
            Value::Match { base, cases } => {
                let (base, old_cases) = (*base, cases.clone());
                self.reduce_match(base, old_cases)
            }
            Value::Opaque { class, id, typee } => {
                let (class, id, typee) = (*class, *id, *typee);
                let typee = self.reduce(typee);
                let value = Value::Opaque { class, id, typee };
                self.gpv(value)
            }
            Value::Substituting {
                base,
                target,
                value,
            } => {
                let (base, target, value) = (*base, *target, *value);
                let rbase = self.reduce(base);
                let rvalue = self.reduce(value);
                let subbed = self.substitute(rbase, target, rvalue);
                if subbed == of {
                    // Hacky way of preventing infinite loops when subbing
                    // things in opaque values.
                    return subbed;
                }
                self.reduce(subbed)
            }
        }
    }

    fn reduce_match(&mut self, base: ValueId, old_cases: Vec<(ValueId, ValueId)>) -> ValueId {
        let base = self.reduce(base);
        let mut cases = Vec::new();
        for (condition, value) in old_cases {
            let condition = self.reduce(condition);
            match self.matches(base, condition) {
                MatchResult::Match { subs } => {
                    if cases.len() == 0 {
                        let mut value = value;
                        for (target, sub) in subs {
                            value = self.substitute(value, target, sub);
                        }
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

    pub fn reduce(&mut self, of: ValueId) -> ValueId {
        if let Some(cached) = self.values[of].cached_reduction {
            cached
        } else {
            self.write_debug_info();
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
            self.write_debug_info();
            let typee = self.get_type(of);
            let rtype = self.get_type(reduced);
            self.write_debug_info();
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
