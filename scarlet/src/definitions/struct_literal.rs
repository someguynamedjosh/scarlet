use std::fmt::{self, Formatter};

use super::{builtin::DBuiltin, hole::DHole, new_type::DNewType, parameter::DParameter};
use crate::item::{
    query::{Query, QueryContext, TypeQuery, TypeCheckQuery, no_type_check_errors},
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
                let r#type = value.query_type(ctx)?;
                fields.push((name.clone(), DParameter::new(r#type).into_ptr()));
            }
            Some(DNewType::new(fields).into_ptr())
        } else {
            Some(DHole::new(DBuiltin::r#type().into_ptr()).into_ptr())
        }
    }

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        if self.is_module {
            no_type_check_errors()
        } else {
            todo!()
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
