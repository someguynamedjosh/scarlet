use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};
impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest_substituting(
        &mut self,
        base: &s2::ItemId,
        target: &Option<s2::ItemId>,
        value: &s2::ItemId,
    ) -> s3::ValueId {
        let base = self.ingest(*base);
        let target = if let Some(target) = target {
            self.resolve_variable(*target)
                .expect("TODO: Nice error, not a variable")
        } else {
            let deps = self.environment.dependencies(base);
            if deps.len() == 0 {
                return base;
            } else {
                deps[0]
            }
        };
        let value = self.ingest(*value);
        self.gpv(s3::Value::Substituting {
            base,
            target,
            value,
        })
    }
}
