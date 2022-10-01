use std::fmt::{self, Formatter};

use crate::item::{
    query::{no_type_check_errors, Query, QueryContext, TypeCheckQuery, TypeQuery},
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

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }
}

impl DHole {
    pub fn new(r#type: ItemPtr) -> Self {
        Self { r#type }
    }
}
