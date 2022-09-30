use std::fmt;

use crate::item::{
    query::{Query, QueryContext, TypeQuery},
    CycleDetectingDebug, Item, ItemDefinition, ItemPtr,
};

pub struct DVariable {
    r#type: ItemPtr,
}

impl CycleDetectingDebug for DVariable {
    fn fmt(&self, f: &mut fmt::Formatter, stack: &[*const Item]) -> fmt::Result {
        write!(f, "ANY ")?;
        self.r#type.fmt(f, stack)
    }
}

impl ItemDefinition for DVariable {
    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(self.r#type.ptr_clone())
    }
}

impl DVariable {
    pub fn new(r#type: ItemPtr) -> Self {
        Self { r#type }
    }
}
