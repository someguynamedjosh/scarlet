use std::collections::HashMap;

use crate::{shared::OpaqueClass, stage2::structure as s2, stage4::structure as s3};

#[derive(Debug)]
pub(super) struct Context<'e, 'i> {
    pub environment: &'e mut s3::Environment,
    pub ingest_map: HashMap<s2::ItemId, s3::ValueId>,
    pub opaque_map: HashMap<s2::OpaqueId, (s3::OpaqueId, s2::ItemId)>,
    pub input: &'i s2::Environment,
    pub stack: Vec<s2::ItemId>,
}

impl<'e, 'i> Context<'e, 'i> {
    /// Get or push value
    pub fn gpv(&mut self, value: s3::Value) -> s3::ValueId {
        self.environment.get_or_push_value(value)
    }

    fn extract_variable(&mut self, from: s3::ValueId) -> Option<s3::OpaqueId> {
        match &self.environment.values[from].value {
            s3::Value::Opaque {
                class: OpaqueClass::Variable,
                id,
                ..
            } => Some(*id),
            _ => None,
        }
    }

    pub fn resolve_variable(&mut self, item: s2::ItemId) -> Option<s3::OpaqueId> {
        let value = self.ingest(item);
        self.extract_variable(value)
    }
}
