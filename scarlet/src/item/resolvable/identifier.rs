use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{environment::Environment, item::ItemPtr, scope::Scope};

#[derive(Clone, Debug)]
pub struct RIdentifier(pub String);

impl Resolvable for RIdentifier {
    fn dyn_clone(&self) -> BoxedResolvable {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        env: &mut Environment,
        this: ItemPtr,
        scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult {
        ResolveResult::Ok(
            scope
                .lookup_ident(env, &self.0)?
                .expect(&format!("Cannot find what {} refers to", self.0))
                .into(),
        )
    }
}
