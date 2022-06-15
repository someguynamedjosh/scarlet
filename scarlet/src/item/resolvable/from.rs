use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        definitions::{substitution::DSubstitution, variable::DVariable},
        ContainmentType, ItemDefinition, ItemPtr,
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
        _this: ItemPtr,
        _scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult {
        let base = self.right.get_from_dex(env);
        let x = env.get_language_item("x").dereference();
        let x = x.downcast_resolved_definition::<DVariable>()?;
        let x_id = x.unwrap().get_variable().ptr_clone();
        let subs = vec![(x_id, self.left.ptr_clone())].into_iter().collect();
        let subbed = DSubstitution::new_unchecked(base.ptr_clone(), subs);
        ResolveResult::Ok(subbed.clone_into_box())
    }

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)> {
        vec![
            (ContainmentType::Computational, &self.left),
            (ContainmentType::Definitional, &self.right),
        ]
    }
}
