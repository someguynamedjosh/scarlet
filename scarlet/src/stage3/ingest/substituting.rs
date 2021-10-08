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
        let mut result = base;
        for (target, value) in substitutions {
            self.ingest_substitution(&mut result, &mut deps, target, value);
        }
        result
    }

    fn ingest_substitution(
        &mut self,
        result: &mut s3::ValueId,
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
        *result = self.gpv(s3::Value::Substituting {
            base: *result,
            target,
            value,
        });
    }
}
