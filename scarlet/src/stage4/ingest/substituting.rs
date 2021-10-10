use super::context::Context;
use crate::{stage3::structure as s3, stage4::structure as s4};
impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest_substituting(
        &mut self,
        base: s3::ValueId,
        substitutions: s3::Substitutions,
    ) -> s4::ValueId {
        let base = self.ingest(base);
        let mut new_subs = s4::Substitutions::new();
        for (target, value) in substitutions {
            self.ingest_substitution(&mut new_subs, target, value);
        }
        let value = s4::Value::Substituting {
            base,
            substitutions: new_subs,
        };
        self.gpv(value)
    }

    fn ingest_substitution(
        &mut self,
        new_subs: &mut s4::Substitutions,
        target: Option<s3::ValueId>,
        value: s3::ValueId,
    ) {
        let target = target.map(|t| self.ingest(t));
        let value = self.ingest(value);
        new_subs.push((target, value));
    }
}
