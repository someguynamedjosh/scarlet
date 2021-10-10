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
}
