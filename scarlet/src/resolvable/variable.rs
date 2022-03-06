use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{
    constructs::{
        variable::{CVariable, Variable},
        ItemDefinition, ItemId,
    },
    environment::Environment,
    scope::Scope,
};

#[derive(Clone, Debug)]
pub struct RVariable {
    pub invariants: Vec<ItemId>,
    pub dependencies: Vec<ItemId>,
}

impl<'x> Resolvable<'x> for RVariable {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        env: &mut Environment<'x>,
        _scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult<'x> {
        let id = env.push_variable(Variable {
            id: None,
            construct: None,
            invariants: self.invariants.clone(),
            dependencies: self.dependencies.clone(),
        });
        let con = CVariable::new(id);
        Ok(ItemDefinition::Resolved(Box::new(con)))
    }
}
