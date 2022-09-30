use std::fmt::{self, Formatter};

use crate::item::{
    query::{Query, QueryContext, TypeQuery},
    CycleDetectingDebug, Item, ItemDefinition, ItemPtr,
};

pub struct DHole {
    r#type: ItemPtr,
}

impl CycleDetectingDebug for DHole {
    fn fmt(&self, f: &mut Formatter, _stack: &[*const Item]) -> fmt::Result {
        write!(f, "_")
    }
}

impl ItemDefinition for DHole {
    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(self.r#type.ptr_clone())
    }
}

impl DHole {
    pub fn new(r#type: ItemPtr) -> Self {
        Self { r#type }
    }
}
