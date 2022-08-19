use crate::item::{definitions::variable::VariableOrder, ItemPtr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Requirement {
    pub statement: ItemPtr,
    pub swallow_dependencies: Vec<ItemPtr>,
    pub order: VariableOrder,
}

impl PartialOrd for Requirement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl Ord for Requirement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl Requirement {
    pub(crate) fn is_same_statement_as(&self, other: &Requirement) -> bool {
        self.statement.is_same_instance_as(&other.statement)
    }
}
