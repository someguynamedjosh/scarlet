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
        let rbase = self.ingest(base);
        self.ingest_map.insert(base, rbase);
        self.ingest_map.insert(input, rbase);
        for (_, def) in &definitions {
            self.ingest(*def);
        }
        rbase
    }
}
