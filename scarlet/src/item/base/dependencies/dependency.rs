use crate::{
    item::definitions::variable::{VariableOrder, VariablePtr},
    util::PtrExtension,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    pub var: VariablePtr,
    pub swallow: Vec<VariablePtr>,
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

impl Dependency {
    pub(crate) fn is_same_variable_as(&self, other: &Dependency) -> bool {
        self.var.is_same_instance_as(&other.var)
    }
}
