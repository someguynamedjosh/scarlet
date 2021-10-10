use super::context::Context;
use crate::{shared, stage2::structure as s3, stage4::structure as s4};

impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest_opaque(
        &mut self,
        &class: &shared::OpaqueClass,
        id: &s3::OpaqueId,
        typee: &s3::ItemId,
    ) -> s4::ValueId {
        let (id, typee) = if let Some(var) = self.opaque_map.get(id) {
            *var
        } else {
            let new_id = self
                .environment
                .opaque_values
                .push(s4::OpaqueValue { stage2_id: *id });
            self.opaque_map.insert(*id, (new_id, *typee));
            (new_id, *typee)
        };
        let typee = self.ingest(typee);
        self.gpv(s4::Value::Opaque { class, id, typee })
    }
}
