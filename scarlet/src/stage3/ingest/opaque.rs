use super::context::Context;
use crate::{shared, stage2::structure as s2, stage3::structure as s3};

impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest_opaque(
        &mut self,
        &class: &shared::OpaqueClass,
        id: &s2::OpaqueId,
        typee: &s2::ItemId,
    ) -> s3::ValueId {
        let (id, typee) = if let Some(var) = self.opaque_map.get(id) {
            *var
        } else {
            let typee = self.child().ingest(*typee);
            let new_id = self
                .environment
                .opaque_values
                .push(s3::OpaqueValue { stage2_id: *id });
            self.opaque_map.insert(*id, (new_id, typee));
            (new_id, typee)
        };
        self.gpv(s3::Value::Opaque { class, id, typee })
    }
}
