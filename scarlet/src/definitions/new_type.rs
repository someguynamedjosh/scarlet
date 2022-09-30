use std::fmt::{self, Formatter};

use super::builtin::DBuiltin;
use crate::item::{
    query::{Query, QueryContext, TypeQuery},
    CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
};

pub struct DNewType {
    fields: Vec<(String, ItemPtr)>,
}

impl CycleDetectingDebug for DNewType {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result {
        write!(f, "NEW_TYPE(\n")?;
        for field in &self.fields {
            write!(
                f,
                "   {} IS {}",
                field.0,
                field.1.to_indented_string(stack, 2)
            )?;
            write!(f, ",\n")?;
        }
        write!(f, ")")
    }
}

impl ItemDefinition for DNewType {
    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(DBuiltin::r#type().into_ptr())
    }
}

impl DNewType {
    pub fn new(fields: Vec<(String, ItemPtr)>) -> Self {
        Self { fields }
    }
}
