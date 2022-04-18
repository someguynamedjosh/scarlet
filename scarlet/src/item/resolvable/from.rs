use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        definitions::{other::DOther, substitution::DSubstitution, variable::DVariable},
        Item, ItemDefinition, ItemPtr,
    },
    scope::Scope,
    util::PtrExtension,
};

#[derive(Clone, Debug)]
pub struct RFrom {
    pub left: ItemPtr,
    pub right: ItemPtr,
}

impl PartialEq for RFrom {
    fn eq(&self, other: &Self) -> bool {
        self.left.is_same_instance_as(&other.left) && self.right.is_same_instance_as(&other.right)
    }
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
        scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult {
        let base = self.right.get_from_dex(env);
        let x = env.get_language_item("x");
        let x = x.downcast_definition::<DVariable>();
        let x_id = x.unwrap().get_variable().ptr_clone();
        let subs = vec![(x_id, self.left.ptr_clone())].into_iter().collect();
        let subbed = DSubstitution::new_unchecked(base.ptr_clone(), subs);
        ResolveResult::Ok(subbed.clone_into_box())
    }
}
