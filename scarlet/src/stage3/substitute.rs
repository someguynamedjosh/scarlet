use super::structure::{Environment, ValueId, VariableId};
use crate::stage3::structure::Value;

impl Environment {
    /// Replaces $target with $value in $base.
    pub fn substitute(&mut self, base: ValueId, target: VariableId, value: ValueId) -> ValueId {
        let base = self.reduce(base);
        match &self.values[base] {
            Value::Any { id, typee } => {
                let (id, typee) = (*id, *typee);
                if id == target {
                    if self.get_type(value) != typee {
                        // todo!("Nice error");
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
