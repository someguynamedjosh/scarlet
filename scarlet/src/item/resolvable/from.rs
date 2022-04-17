use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        definitions::{substitution::DSubstitution, variable::DVariable},
        ItemDefinition, ItemPtr,
    },
    scope::Scope,
};

#[derive(Clone, Debug)]
pub struct RFrom {
    pub left: ItemPtr,
    pub right: ItemPtr,
}

impl_any_eq_from_regular_eq!(RFrom);

impl Resolvable for RFrom {
    fn dyn_clone(&self) -> BoxedResolvable {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        env: &mut Environment,
        this: ItemPtr,
        _scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult {
        let base = env.create_from_dex(self.right)?;
        let x = env.get_language_item("x");
        let x = env.get_and_downcast_construct_definition::<DVariable>(x)?;
        let x_id = x.unwrap().get_id();
        let subs = vec![(x_id, self.left)].into_iter().collect();
        let subbed = DSubstitution::new_unchecked(base, base, subs);
        ResolveResult::Ok(ItemDefinition::Resolved(Box::new(subbed)))
    }
}
