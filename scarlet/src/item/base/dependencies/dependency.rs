use crate::item::definitions::variable::{VariableId, VariableOrder};


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    pub id: VariableId,
    pub swallow: Vec<VariableId>,
    pub order: VariableOrder,
}

impl PartialOrd for Dependency {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl Ord for Dependency {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}