use super::structure::{Environment, ValueId, VariableId};
use crate::stage3::structure::Value;

impl Environment {
    /// Replaces $target with $value in $base.
    pub fn substitute(&mut self, base: ValueId, target: VariableId, value: ValueId) -> ValueId {
        match &self.values[base] {
            Value::Any(variable) => {
                if *variable == target {
                    value
                } else {
                    base
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
                self.substitute(sub_base, other_target, sub_value)
            }
            Value::Variant(_) => todo!(),
        }
    }
}
