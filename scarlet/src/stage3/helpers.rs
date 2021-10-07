use super::structure::{Environment, OpaqueId, ValueId, Variables};
use crate::{stage2::structure::BuiltinValue, stage3::structure::Value};

impl Environment {
    pub fn gp_builtin_value(&mut self, value: BuiltinValue) -> ValueId {
        self.gpv(Value::BuiltinValue(value))
    }

    pub fn gp_origin_type(&mut self) -> ValueId {
        self.gp_builtin_value(BuiltinValue::OriginType)
    }

    pub fn gp_u8_type(&mut self) -> ValueId {
        self.gp_builtin_value(BuiltinValue::U8Type)
    }

    /// Get or push value
    pub fn gpv(&mut self, value: Value) -> ValueId {
        self.get_or_push_value(value)
    }

    pub fn remove_from_variable(&mut self, inn: ValueId, variable_to_remove: OpaqueId) -> ValueId {
        match &self.values[inn].value {
            Value::From { base, variable } => {
                let (base, variable) = (*base, *variable);
                let base = self.remove_from_variable(base, variable_to_remove);
                if variable == variable_to_remove {
                    base
                } else {
                    self.gpv(Value::From { base, variable })
                }
            }
            _ => inn,
        }
    }

    fn get_from_variables_impl(&self, typee: ValueId, vars: &mut Variables) {
        match &self.values[typee].value {
            Value::From { base, variable } => {
                let (base, variable) = (*base, *variable);
                self.get_from_variables_impl(base, vars);
                assert!(!vars.contains_key(&variable));
                vars.insert_no_replace(variable, ());
            }
            Value::Substituting {
                base,
                target,
                value,
            } => todo!(),
            _ => (),
        }
    }

    pub fn get_from_variables(&self, typee: ValueId) -> Variables {
        let mut result = Variables::new();
        self.get_from_variables_impl(typee, &mut result);
        result
    }

    pub fn with_from_variables(&mut self, base: ValueId, variables: &[OpaqueId]) -> ValueId {
        if variables.len() == 0 {
            base
        } else {
            let variable = variables[0];
            let base = self.with_from_variables(base, &variables[1..]);
            self.gpv(Value::From { base, variable })
        }
    }

    pub fn dependencies(&mut self, value: ValueId) -> Vec<OpaqueId> {
        let value_type = self.get_type(value);
        let value_deps = self.get_from_variables(value_type);
        value_deps.into_iter().map(|i| i.0).collect()
    }
}
