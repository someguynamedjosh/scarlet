use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;

use super::{builtin::DBuiltin, parameter::ParameterPtr};
use crate::item::{
    query::{
        no_type_check_errors, ChildrenQuery, ParametersQuery, Query, QueryContext, TypeCheckQuery,
        TypeQuery,
    },
    type_hints::TypeHint,
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
    fn collect_children(&self, into: &mut Vec<ItemPtr>) {
        for (_, ty) in &self.fields {
            ty.collect_self_and_children(into);
        }
    }

    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![]
    }

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

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        this.ptr_clone()
    }
}

impl DNewType {
    pub fn new(fields: Vec<(String, ItemPtr)>) -> Self {
        Self { fields }
    }
}
