use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{environment::Environment, scope::Scope};

#[derive(Clone, Debug)]
pub struct RIdentifier<'x>(pub &'x str);

impl<'x> Resolvable<'x> for RIdentifier<'x> {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        env: &mut Environment<'x>,
        scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult<'x> {
        Ok(scope
            .lookup_ident(env, self.0)?
            .expect(&format!("Cannot find what {} refers to", self.0))
            .into())
    }
}
