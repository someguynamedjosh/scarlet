use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{
    constructs::{
        variable::{CVariable, Variable, VariableOrder},
        ItemDefinition, ItemId,
    },
    environment::Environment,
    scope::Scope,
};

#[derive(Clone, Debug)]
pub struct RVariable {
    pub invariants: Vec<ItemId>,
    pub dependencies: Vec<ItemId>,
    pub order: VariableOrder,
}

impl<'x> Resolvable<'x> for RVariable {
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
        let id = env.push_variable(Variable {
            id: None,
            item: None,
            invariants: self.invariants.clone(),
            dependencies: self.dependencies.clone(),
            order: self.order.clone(),
        });
        let con = CVariable::new(id);
        ResolveResult::Ok(ItemDefinition::Resolved(Box::new(con)))
    }
}
