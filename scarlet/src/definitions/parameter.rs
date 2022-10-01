use std::fmt;

use crate::item::{
    query::{Query, QueryContext, TypeQuery, TypeCheckQuery},
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

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        todo!()
    }

    
}

impl DParameter {
    pub fn new(r#type: ItemPtr) -> Self {
        Self { r#type }
    }
}
