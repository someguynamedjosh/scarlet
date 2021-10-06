use super::structure::{Environment, ValueId, VariableId};
use crate::stage3::structure::Value;

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

    /// Replaces $target with $value in $base.
    pub fn substitute(&mut self, base: ValueId, target: VariableId, value: ValueId) -> ValueId {
        let base = self.reduce(base);
        match &self.values[base].value {
            Value::Any { id, typee } => {
                let (id, typee) = (*id, *typee);
                if id == target {
                    let value_type = self.get_type(value);
                    if !self.type_is_base_of_other(typee, value_type) {
                        todo!("Nice error, {:?} is not base of {:?}", value_type, typee);
                    }
                    value
                } else {
                    let typee = self.substitute(typee, target, value);
                    let value = Value::Any { id, typee };
                    self.gpv(value)
                }
            }
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(..) => base,
            Value::From { base, variable } => {
                let (base, variable) = (*base, *variable);
                let base = self.substitute(base, target, value);
                if variable == target {
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
            Value::Substituting {
                base: other_base,
                target: other_target,
                value: other_value,
            } => {
                let (other_base, other_target, other_value) =
                    (*other_base, *other_target, *other_value);
                let sub_base = self.substitute(other_base, target, value);
                let sub_value = self.substitute(other_value, target, value);
                let value = Value::Substituting {
                    base: sub_base,
                    target: other_target,
                    value: sub_value,
                };
                self.gpv(value)
            }
            Value::Variant(_) => todo!(),
        }
    }
}
