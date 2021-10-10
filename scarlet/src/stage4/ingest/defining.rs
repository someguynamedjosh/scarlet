use super::context::Context;
use crate::{stage2::structure as s3, stage4::structure as s4};
impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest_defining(
        &mut self,
        definitions: &s3::Definitions,
        base: &s3::ItemId,
        input: s3::ItemId,
    ) -> s4::ValueId {
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
