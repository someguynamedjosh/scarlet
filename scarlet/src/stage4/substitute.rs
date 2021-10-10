use super::structure::{Environment, Substitutions, ValueId};
use crate::{shared::OpaqueClass, stage4::structure::Value};

impl Environment {
    pub fn type_is_base_of_other(&self, base_id: ValueId, other_id: ValueId) -> bool {
        let base = &self.values[base_id].value;
        let other = &self.values[other_id].value;
        match (base, other) {
            (
                Value::From {
                    base: base_base,
                    variable: base_variable,
                },
                Value::From {
                    base: other_base,
                    variable: other_variable,
                },
            ) => {
                if base_variable == other_variable {
                    self.type_is_base_of_other(*base_base, *other_base)
                } else {
                    self.type_is_base_of_other(base_id, *other_base)
                }
            }
            (
                _,
                Value::From {
                    base: other_base, ..
                },
            ) => self.type_is_base_of_other(base_id, *other_base),
            _ => base_id == other_id,
        }
    }

    pub fn substitute(&mut self, base: ValueId, substitutions: &Substitutions) -> ValueId {
        let base = self.reduce(base);
        match self.values[base].value.clone() {
            Value::Opaque { class, id, typee } => {
                if let Some(&value) = substitutions.get(&id) {
                    debug_assert_eq!(class, OpaqueClass::Variable);
                    let value_type = self.get_type(value);
                    if !self.type_is_base_of_other(typee, value_type) {
                        println!("{:#?}", self);
                        todo!("Nice error, {:?} is not base of {:?}", value_type, typee);
                    }
                    value
                } else {
                    let from_vars = self.get_from_variables(typee);
                    let mut keep_subs = Substitutions::new();
                    for &(target, value) in substitutions {
                        if from_vars.contains_key(&target) {
                            keep_subs.insert_no_replace(target, value);
                        }
                    }
                    if keep_subs.len() == 0 {
                        base
                    } else {
                        let value = Value::Substituting {
                            base,
                            substitutions: keep_subs,
                        };
                        self.gpv(value)
                    }
                }
            }
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(..) => base,
            Value::From { base, variable } => {
                let base = self.substitute(base, substitutions);
                if let Some(&value) = substitutions.get(&variable) {
                    let value_deps = self.dependencies(value);
                    let already_included = self.get_from_variables(base);
                    let value_deps: Vec<_> = value_deps
                        .into_iter()
                        .filter(|dep| !already_included.contains_key(dep))
                        .collect();
                    self.with_from_variables(base, &value_deps[..])
                } else {
                    self.gpv(Value::From { base, variable })
                }
            }
            Value::Match {
                base,
                cases: old_cases,
            } => {
                let base = self.substitute(base, substitutions);
                let mut cases = Vec::new();
                for (condition, case_value) in old_cases {
                    // Don't substitute condition because it can bind variables.
                    let case_value = self.substitute(case_value, substitutions);
                    cases.push((condition, case_value))
                }
                self.gpv(Value::Match { base, cases })
            }
            Value::SelfReference { .. } => todo!(),
            Value::Substituting {
                base: other_base,
                substitutions: other_substitutions,
            } => {
                let subbed_base = self.substitute(other_base, substitutions);
                let mut subbed_subs = Substitutions::new();
                for (target, value) in other_substitutions {
                    let value = self.substitute(value, substitutions);
                    subbed_subs.insert_no_replace(target, value);
                }
                let subbed_base_val = self.values[subbed_base].value.clone();
                if let Value::Substituting {
                    base,
                    mut substitutions,
                } = subbed_base_val
                {
                    for (target, value) in subbed_subs {
                        if !substitutions.contains_key(&target) {
                            substitutions.insert_no_replace(target, value);
                        }
                    }
                    self.gpv(Value::Substituting {
                        base,
                        substitutions,
                    })
                } else {
                    self.gpv(Value::Substituting {
                        base: subbed_base,
                        substitutions: subbed_subs,
                    })
                }
            }
        }
    }
}
