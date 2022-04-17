use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{
    item::{
        definitions::variable::{DVariable, Variable, VariableOrder},
        ItemDefinition, ItemPtr,
    },
    environment::Environment,
    scope::Scope,
};

#[derive(Clone, Debug)]
pub struct RVariable {
    pub invariants: Vec<ItemPtr>,
    pub dependencies: Vec<ItemPtr>,
    pub order: VariableOrder,
}

impl Resolvable for RVariable {
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
        let id = env.push_variable(Variable {
            id: None,
            item: None,
            invariants: self.invariants.clone(),
            dependencies: self.dependencies.clone(),
            order: self.order.clone(),
        });
        let con = DVariable::new(id);
        ResolveResult::Ok(ItemDefinition::Resolved(Box::new(con)))
    }
}
