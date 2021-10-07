use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};

impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest_any(&mut self, id: &s2::VariableId, typee: &s2::ItemId) -> s3::ValueId {
        let (id, typee) = if let Some(var) = self.variable_map.get(id) {
            *var
        } else {
            let typee = self.child().without_path().ingest(*typee);
            let new_id = self
                .environment
                .variables
                .push(s3::Variable { stage2_id: *id });
            self.variable_map.insert(*id, (new_id, typee));
            (new_id, typee)
        };
        self.gpv(s3::Value::Any { id, typee })
    }

    pub fn ingest_variant(&mut self, id: &s2::VariantId, typee: &s2::ItemId) -> s3::ValueId {
        let (id, typee) = if let Some(vnt) = self.variant_map.get(id) {
            *vnt
        } else {
            let typee = self.child().without_path().ingest(*typee);
            let new_id = self
                .environment
                .variants
                .push(s3::Variant { stage2_id: *id });
            self.variant_map.insert(*id, (new_id, typee));
            (new_id, typee)
        };
        self.gpv(s3::Value::Variant { id, typee })
    }
}
