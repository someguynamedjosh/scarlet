use std::collections::HashMap;

use crate::{shared::OpaqueClass, stage3::structure as s3, stage4::structure as s4};

#[derive(Debug)]
pub(super) struct Context<'e, 'i> {
    pub environment: &'e mut s4::Environment,
    pub ingest_map: HashMap<s3::ValueId, s4::ValueId>,
    pub opaque_map: HashMap<s3::OpaqueId, (s4::OpaqueId, s3::ValueId)>,
    pub input: &'i s3::Environment,
    pub stack: Vec<s3::ValueId>,
}

impl<'e, 'i> Context<'e, 'i> {
    /// Get or push value
    pub fn gpv(&mut self, value: s4::Value) -> s4::ValueId {
        self.environment.get_or_push_value(value)
    }

    fn extract_variable(&mut self, from: s4::ValueId) -> Option<s4::OpaqueId> {
        match &self.environment.values[from].value {
            s4::Value::Opaque {
                class: OpaqueClass::Variable,
                id,
                ..
            } => Some(*id),
            _ => None,
        }
    }

    pub fn resolve_variable(&mut self, item: s3::ValueId) -> Option<s4::OpaqueId> {
        let value = self.ingest(item);
        self.extract_variable(value)
    }
}
