use super::{
    base::{Construct, ItemId},
    substitution::Substitutions,
    variable::VariableId,
};
use crate::{
    environment::{dependencies::DepResult, Environment},
    impl_any_eq_for_construct,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CRecursion(ItemId);

impl CRecursion {
    pub fn new<'x>(base: ItemId) -> Self {
        Self(base)
    }

    pub(crate) fn get_base(&self) -> ItemId {
        self.0
    }
}

impl_any_eq_for_construct!(CRecursion);

impl Construct for CRecursion {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        env.get_dependencies(self.0)
    }
}
