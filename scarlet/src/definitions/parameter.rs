use std::fmt;

use crate::item::{
    query::{Query, QueryContext, TypeQuery},
    CycleDetectingDebug, Item, ItemDefinition, ItemPtr,
};

pub struct DParameter {
    r#type: ItemPtr,
}

impl CycleDetectingDebug for DParameter {
    fn fmt(&self, f: &mut fmt::Formatter, stack: &[*const Item]) -> fmt::Result {
        write!(f, "ANY ")?;
        self.r#type.fmt(f, stack)
    }
}

impl ItemDefinition for DParameter {
    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(self.r#type.ptr_clone())
    }
}

impl DParameter {
    pub fn new(r#type: ItemPtr) -> Self {
        Self { r#type }
    }
}
