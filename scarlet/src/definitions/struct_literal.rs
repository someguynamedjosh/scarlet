use std::fmt::{self, Formatter};

use crate::item::{CycleDetectingDebug, Item, ItemDefinition, ItemPtr};

pub struct DStructLiteral {
    fields: Vec<(String, ItemPtr)>,
}

impl CycleDetectingDebug for DStructLiteral {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result {
        write!(f, "[")?;
        for field in &self.fields {
            write!(f, "{} IS ", field.0)?;
            field.1.fmt(f, stack)?;
            write!(f, ",")?;
        }
        write!(f, "]")
    }
}

impl ItemDefinition for DStructLiteral {}

impl DStructLiteral {
    pub fn new(fields: Vec<(String, ItemPtr)>) -> Self {
        Self { fields }
    }
}
