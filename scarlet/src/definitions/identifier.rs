use std::fmt;

use crate::item::{CycleDetectingDebug, Item, ItemDefinition};

pub struct DIdentifier {
    identifier: String,
}

impl CycleDetectingDebug for DIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter, _stack: &[*const Item]) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl ItemDefinition for DIdentifier {}

impl DIdentifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}
