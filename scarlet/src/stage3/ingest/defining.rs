use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};
impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest_defining(
        &mut self,
        definitions: &s2::Definitions,
        base: &s2::ItemId,
        input: s2::ItemId,
    ) -> s3::ValueId {
        let mut child = self.child().with_additional_parent_scope(definitions);
        let (base, definitions) = (*base, definitions.clone());
        let rbase = child.ingest(base);
        self.ingest_map.insert(base, rbase);
        self.ingest_map.insert(input, rbase);
        let mut child = self.child().with_additional_parent_scope(&definitions);
        for (name, def) in &definitions {
            child
                .child()
                .with_additional_path_component(s3::PathComponent::Member(name.clone()))
                .ingest(*def);
        }
        rbase
    }
}
