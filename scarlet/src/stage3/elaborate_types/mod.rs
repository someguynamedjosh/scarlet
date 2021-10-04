use super::structure::{Environment, ValueId};
use crate::{stage2::structure::BuiltinValue, stage3::structure::Value};

impl Environment {
    fn gp_builtin_value(&mut self, value: BuiltinValue) -> ValueId {
        self.values.get_or_push(Value::BuiltinValue(value))
    }

    fn gp_origin_type(&mut self) -> ValueId {
        self.gp_builtin_value(BuiltinValue::OriginType)
    }

    fn gp_u8_type(&mut self) -> ValueId {
        self.gp_builtin_value(BuiltinValue::U8Type)
    }

    fn elaborate_type_from_scratch(&mut self, of: ValueId) -> ValueId {
        match &self.values[of] {
            Value::Any(var) => self.variables[*var].typee,
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(val) => match val {
                BuiltinValue::OriginType | &BuiltinValue::U8Type => self.gp_origin_type(),
                BuiltinValue::U8(..) => self.gp_u8_type(),
            },
            Value::From { base, values } => todo!(),
            Value::Substituting {
                base,
                substitutions,
            } => todo!(),
            Value::Variant(_) => todo!(),
        }
    }

    pub fn get_type(&mut self, of: ValueId) -> ValueId {
        if let Some(cached) = self.type_cache.get(&of) {
            *cached
        } else {
            let typee = self.elaborate_type_from_scratch(of);
            let typee = self.reduce(typee);
            self.type_cache.insert(of, typee);
            typee
        }
    }
}
