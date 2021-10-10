use super::structure::{Environment, OpaqueId, ValueId, Variables};
use crate::{stage2::structure::BuiltinValue, stage4::structure::Value};

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

    pub fn remove_from_variables(
        &mut self,
        inn: ValueId,
        variables_to_remove: Variables,
    ) -> ValueId {
        let mut result = inn;
        for (var, _) in variables_to_remove {
            result = self.remove_from_variable(result, var);
        }
        result
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

    fn split_from_type_impl(&self, typee: ValueId, vars: &mut Variables) -> ValueId {
        match &self.values[typee].value {
            &Value::From { base, variable } => {
                vars.insert_no_replace(variable, ());
                self.split_from_type_impl(base, vars);
                base
            }
            _ => typee,
        }
    }

    pub fn split_from_type(&self, typee: ValueId) -> (ValueId, Variables) {
        let mut result = Variables::new();
        let base = self.split_from_type_impl(typee, &mut result);
        (base, result)
    }

    pub fn get_from_variables(&self, typee: ValueId) -> Variables {
        self.split_from_type(typee).1
    }

    pub fn with_from_variables(&mut self, base: ValueId, variables: &[OpaqueId]) -> ValueId {
        if variables.len() == 0 {
            base
        } else {
            let base = self.with_from_variables(base, &variables[1..]);
            let existing_vars = self.get_from_variables(base);
            let variable = variables[0];
            if existing_vars.contains_key(&variable) {
                base
            } else {
                self.gpv(Value::From { base, variable })
            }
        }
    }

    pub fn dependencies(&mut self, value: ValueId) -> Vec<OpaqueId> {
        let value_type = self.get_type(value);
        let value_deps = self.get_from_variables(value_type);
        value_deps.into_iter().map(|i| i.0).collect()
    }
}
