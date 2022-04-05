use super::{downcast_construct, substitution::Substitutions, Construct, ItemId};
use crate::{
    environment::{dependencies::DepResult, sub_expr::NestedSubstitutions, Environment},
    impl_any_eq_for_construct,
    scope::{LookupIdentResult, LookupInvariantError, ReverseLookupIdentResult, Scope},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CDecision {
    left: ItemId,
    right: ItemId,
    equal: ItemId,
    unequal: ItemId,
}

impl CDecision {
    pub fn new<'x>(left: ItemId, right: ItemId, equal: ItemId, unequal: ItemId) -> Self {
        Self {
            left,
            right,
            equal,
            unequal,
        }
    }

    pub fn left(&self) -> ItemId {
        self.left
    }

    pub fn right(&self) -> ItemId {
        self.right
    }

    pub fn equal(&self) -> ItemId {
        self.equal
    }

    pub fn unequal(&self) -> ItemId {
        self.unequal
    }
}

impl_any_eq_for_construct!(CDecision);

impl Construct for CDecision {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn contents<'x>(&self) -> Vec<ItemId> {
        vec![self.left, self.right, self.equal, self.unequal]
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let mut deps = env.get_dependencies(self.left);
        deps.append(env.get_dependencies(self.right));
        deps.append(env.get_dependencies(self.equal));
        deps.append(env.get_dependencies(self.unequal));
        deps
    }
}
