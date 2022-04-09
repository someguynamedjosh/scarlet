use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{environment::Environment, scope::Scope, constructs::ItemId};

#[derive(Clone, Debug)]
pub struct RPlaceholder;

impl<'x> Resolvable<'x> for RPlaceholder {
    fn is_placeholder(&self) -> bool {
        true
    }

    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        env: &mut Environment<'x>,
        this: ItemId,
        _scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult {
        eprintln!("{:#?}", env);
        unreachable!()
    }
}
