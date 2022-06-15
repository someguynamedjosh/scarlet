use super::{BoxedResolvable, Resolvable, ResolveResult};
use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        definitions::{
            other::DOther,
            variable::{DVariable, Variable, VariableOrder},
        },
        ContainmentType, ItemDefinition, ItemPtr,
    },
    scope::Scope,
};

#[derive(Clone, Debug)]
pub struct RVariable {
    pub invariants: Vec<ItemPtr>,
    pub dependencies: Vec<ItemPtr>,
    pub order: VariableOrder,
}

impl PartialEq for RVariable {
    fn eq(&self, other: &Self) -> bool {
        self.invariants
            .iter()
            .zip(other.invariants.iter())
            .all(|(this, other)| this.is_same_instance_as(other))
            && self
                .dependencies
                .iter()
                .zip(other.dependencies.iter())
                .all(|(this, other)| this.is_same_instance_as(other))
            && self.invariants.len() == other.invariants.len()
            && self.dependencies.len() == other.dependencies.len()
            && self.order == other.order
    }
}

impl_any_eq_from_regular_eq!(RVariable);

impl Resolvable for RVariable {
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
        let id = DVariable::new(
            self.invariants.clone(),
            self.dependencies.clone(),
            self.order.clone(),
            scope,
        );
        ResolveResult::Ok(DOther::new(id).clone_into_box())
    }

    fn contents(&self) -> Vec<(crate::item::ContainmentType, &ItemPtr)> {
        let mut result = vec![];
        for inv in &self.invariants {
            result.push((ContainmentType::Definitional, inv));
        }
        for dep in &self.dependencies {
            result.push((ContainmentType::Definitional, dep));
        }
        result
    }
}
