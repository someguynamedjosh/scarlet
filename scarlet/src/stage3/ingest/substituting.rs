use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};
impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest_substituting(
        &mut self,
        base: &s2::ItemId,
        substitutions: &s2::Substitutions,
    ) -> s3::ValueId {
        let base = self.ingest(*base);
        let mut deps = self.environment.dependencies(base);
        let mut new_subs = s3::Substitutions::new();
        for (target, value) in substitutions {
            self.ingest_substitution(&mut new_subs, &mut deps, target, value);
        }
        let value = s3::Value::Substituting {
            base,
            substitutions: new_subs,
        };
        self.gpv(value)
    }

    fn ingest_substitution(
        &mut self,
        new_subs: &mut s3::Substitutions,
        deps: &mut Vec<s3::OpaqueId>,
        target: &Option<s2::ItemId>,
        value: &s2::ItemId,
    ) {
        let target = if let Some(target) = target {
            self.resolve_variable(*target)
                .expect("TODO: Nice error, not a variable")
        } else {
            if deps.len() == 0 {
                return;
            } else {
                let target = deps[0];
                deps.remove(0);
                target
            }
        };
        let value = self.ingest(*value);
        if new_subs.contains_key(&target) {
            todo!("Nice error, same var replaced twice.");
        }
        new_subs.insert_no_replace(target, value);
    }
}
