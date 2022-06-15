use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{definitions::other::DOther, ContainmentType, ItemDefinition, ItemPtr},
    scope::Scope,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RIdentifier(pub String);

impl_any_eq_from_regular_eq!(RIdentifier);

impl Resolvable for RIdentifier {
    fn dyn_clone(&self) -> BoxedResolvable {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        _env: &mut Environment,
        _this: ItemPtr,
        scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult {
        let identified = scope
            .lookup_ident(&self.0)?
            .expect(&format!("Cannot find what {} refers to", self.0));
        ResolveResult::Ok(DOther::new(identified).clone_into_box())
    }

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)> {
        vec![]
    }
}
