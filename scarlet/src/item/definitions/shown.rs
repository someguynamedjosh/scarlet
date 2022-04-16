use crate::item::{
    base::{ItemDefinition, ItemPtr},
    substitution::Substitutions,
    variable::VariableId,
    GenInvResult,
};
use crate::{
    environment::{
        dependencies::DepResult,
        discover_equality::{DeqResult, DeqSide},
        Environment,
    },
    impl_any_eq_for_construct,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CShown(ItemPtr);

impl CShown {
    pub fn new(base: ItemPtr) -> Self {
        Self(base)
    }

    pub(crate) fn get_base(&self) -> ItemPtr {
        self.0
    }
}

impl_any_eq_for_construct!(CShown);

impl ItemDefinition for CShown {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<ItemPtr> {
        vec![self.0]
    }

    fn get_dependencies(&self, env: &mut Environment) -> DepResult {
        env.get_dependencies(self.0)
    }

    fn generated_invariants(&self, _this: ItemPtr, env: &mut Environment) -> GenInvResult {
        env.generated_invariants(self.0)
    }

    fn dereference(
        &self,
        env: &mut Environment,
    ) -> Option<(ItemPtr, Option<&Substitutions>, Option<Vec<ItemPtr>>)> {
        Some((self.0, None, None))
    }
}
