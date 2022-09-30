use std::fmt;

use crate::item::{
    query::{Query, QueryContext, TypeQuery},
    CycleDetectingDebug, Item, ItemDefinition,
};

pub struct DIdentifier {
    identifier: String,
}

impl CycleDetectingDebug for DIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter, _stack: &[*const Item]) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl ItemDefinition for DIdentifier {
    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        None
    }
}

impl DIdentifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}
