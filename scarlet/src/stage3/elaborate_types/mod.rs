use super::structure::{Environment, ValueId, VariableId, Variables};
use crate::{stage2::structure::BuiltinValue, stage3::structure::Value};

impl Environment {
    fn elaborate_type_from_scratch(&mut self, of: ValueId) -> ValueId {
        match &self.values[of].value {
            Value::Any { id, typee } => {
                let (variable, typee) = (*id, *typee);
                let type_deps = self.dependencies(typee);
                let base = self.with_from_variables(typee, &type_deps[..]);
                self.gpv(Value::From { base, variable })
            }
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(val) => match val {
                BuiltinValue::OriginType | &BuiltinValue::U8Type => self.gp_origin_type(),
                BuiltinValue::U8(..) => self.gp_u8_type(),
            },
            Value::From { base, variable } => {
                let (base, variable) = (*base, *variable);
                let base_type = self.get_type(base);
                self.remove_from_variable(base_type, variable)
            }
            Value::Substituting {
                base,
                target,
                value,
            } => {
                let (base, target, value) = (*base, *target, *value);
                let base_type = self.get_type(base);
                self.substitute(base_type, target, value)
            }
            Value::Variant { typee, .. } => {
                let typee = *typee;
                let type_deps = self.dependencies(typee);
                self.with_from_variables(typee, &type_deps[..])
            }
        }
    }

    pub fn get_type(&mut self, of: ValueId) -> ValueId {
        if let Some(cached) = self.values[of].cached_type {
            cached
        } else {
            let typee = self.elaborate_type_from_scratch(of);
            let typee = self.reduce(typee);
            self.values[of].cached_type = Some(typee);
            typee
        }
    }
}
