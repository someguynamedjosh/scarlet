use std::fmt::{self, Formatter};

use super::builtin::DBuiltin;
use crate::item::{
    query::{
        no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
    },
    CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
};

#[derive(Clone)]
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
    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(DBuiltin::r#type().into_ptr())
    }

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }
}

impl DNewType {
    pub fn new(fields: Vec<(String, ItemPtr)>) -> Self {
        Self { fields }
    }
}
