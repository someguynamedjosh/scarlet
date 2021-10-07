use super::structure::{Environment, Value, ValueId};
use crate::shared::OpaqueClass;

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

    fn matches(&mut self, base: ValueId, condition: ValueId) -> Option<bool> {
        let base_def = &self.values[base];
        let condition_def = &self.values[condition];
        match (&base_def.value, &condition_def.value) {
            (
                Value::Opaque {
                    class: OpaqueClass::Variant,
                    id: base_id,
                    ..
                },
                Value::Opaque {
                    class: OpaqueClass::Variant,
                    id: condition_id,
                    ..
                },
            ) => Some(base_id == condition_id),
            _ => None,
        }
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
                let base = self.reduce(base);
                let mut cases = Vec::new();
                for (condition, value) in old_cases {
                    let condition = self.reduce(condition);
                    match self.matches(base, condition) {
                        Some(true) => {
                            if cases.len() == 0 {
                                return self.reduce(value);
                            } else {
                                let value = self.reduce(value);
                                cases.push((condition, value));
                                break;
                            }
                        }
                        Some(false) => (),
                        None => {
                            let value = self.reduce(value);
                            cases.push((condition, value));
                        }
                    }
                }
                self.gpv(Value::Match { base, cases })
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
            debug_assert_eq!(typee, self.get_type(reduced));
            self.values[reduced].cached_type = Some(typee);
            reduced
        }
    }
}
