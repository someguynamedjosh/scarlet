use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};
impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest_defining(
        &mut self,
        definitions: &s2::Definitions,
        base: &s2::ItemId,
        input: s2::ItemId,
    ) -> s3::ValueId {
        let (base, definitions) = (*base, definitions.clone());
        let mut child = self.child().with_additional_parent_scope(&definitions);
        let rbase = child.ingest(base);
        self.ingest_map.insert(base, rbase);
        self.ingest_map.insert(input, rbase);
        let mut child = self.child().with_additional_parent_scope(&definitions);
        for (_, def) in &definitions {
            child.ingest(*def);
        }
        rbase
    }
}
