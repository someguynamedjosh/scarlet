use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{builtin::DBuiltin, compound_type::DCompoundType, parameter::ParameterPtr};
use crate::item::{CddContext, CycleDetectingDebug, ItemDefinition, ItemRef};

#[derive(Clone)]
pub struct DHole<Definition, Analysis> {
    r#type: ItemRef<Definition, Analysis>,
}

impl<Definition, Analysis> CycleDetectingDebug for DHole<Definition, Analysis> {
    fn fmt(&self, f: &mut Formatter, _ctx: &mut CddContext) -> fmt::Result {
        write!(f, "_")
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DHole<Definition, Analysis>
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        vec![self.r#type.ptr_clone()]
    }
}

impl<Definition, Analysis> DHole<Definition, Analysis> {
    pub fn new(r#type: ItemRef<Definition, Analysis>) -> Self {
        Self { r#type }
    }
}
