use std::fmt::{self, Formatter};

use super::{builtin::DBuiltin, hole::DHole, new_type::DNewType};
use crate::item::{
    query::{Query, QueryContext, TypeQuery},
    CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
};

pub struct DStructLiteral {
    fields: Vec<(String, ItemPtr)>,
    /// If true, a type is automatically generated based on the contents. If
    /// false, the type should be inferred.
    is_module: bool,
}

impl CycleDetectingDebug for DStructLiteral {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result {
        write!(f, "[\n")?;
        for field in &self.fields {
            write!(
                f,
                "   {} IS {}",
                field.0,
                field.1.to_indented_string(stack, 2)
            )?;
            write!(f, ",\n")?;
        }
        write!(f, "]")
    }
}

impl ItemDefinition for DStructLiteral {
    fn recompute_type(&self, ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        if self.is_module {
            let mut fields = Vec::new();
            for (name, value) in &self.fields {
                fields.push((name.clone(), value.query_type(ctx)?));
            }
            Some(DNewType::new(fields).into_ptr())
        } else {
            Some(DHole::new(DBuiltin::r#type().into_ptr()).into_ptr())
        }
    }
}

impl DStructLiteral {
    pub fn new_module(fields: Vec<(String, ItemPtr)>) -> Self {
        Self {
            fields,
            is_module: true,
        }
    }

    pub fn new_struct(fields: Vec<(String, ItemPtr)>) -> Self {
        Self {
            fields,
            is_module: false,
        }
    }
}
