use super::structure::{Environment, ValueId, Variables};
use crate::{shared::OpaqueClass, stage2::structure::BuiltinValue, stage3::structure::Value};

impl Environment {
    fn elaborate_type_from_scratch(&mut self, of: ValueId) -> ValueId {
        match self.values[of].value.clone() {
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(val) => match val {
                BuiltinValue::OriginType | BuiltinValue::U8Type => self.gp_origin_type(),
                BuiltinValue::U8(..) => self.gp_u8_type(),
            },
            Value::From { base, variable } => {
                let base_type = self.get_type(base);
                self.remove_from_variable(base_type, variable)
            }
            Value::Match { base, cases } => {
                let base_type = self.get_type(base);
                let mut variables = self.get_from_variables(base_type);
                let mut the_value_type = None;
                for (condition, value) in cases {
                    // TODO: type check
                    let condition_type = self.get_type(condition);
                    let value_type = self.get_type(value);
                    variables = variables.union(self.get_from_variables(value_type));
                    the_value_type = Some(value_type)
                }
                // TODO: Never type.
                let value_type = the_value_type.unwrap();
                let variables: Vec<_> = variables.into_iter().map(|x| x.0).collect();
                self.with_from_variables(value_type, &variables[..])
            }
            Value::Opaque { class, id, typee } => {
                let type_deps = self.dependencies(typee);
                let base = self.with_from_variables(typee, &type_deps[..]);
                if class == OpaqueClass::Variable {
                    let variable = id;
                    self.gpv(Value::From { base, variable })
                } else {
                    base
                }
            }
            Value::Substituting {
                base,
                substitutions,
            } => {
                let base_type = self.get_type(base);
                self.substitute(base_type, &substitutions)
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
