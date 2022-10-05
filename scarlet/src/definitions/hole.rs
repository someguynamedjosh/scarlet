use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{builtin::DBuiltin, parameter::ParameterPtr};
use crate::{
    environment::Environment,
    item::{
        query::{
            no_type_check_errors, ChildrenQuery, ParametersQuery, Query, QueryContext,
            TypeCheckQuery, TypeQuery,
        },
        type_hints::TypeHint,
        CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone)]
pub struct DHole {
    r#type: ItemPtr,
}

impl CycleDetectingDebug for DHole {
    fn fmt(&self, f: &mut Formatter, _stack: &[*const Item]) -> fmt::Result {
        write!(f, "_")
    }
}

impl ItemDefinition for DHole {
    fn children(&self) -> Vec<ItemPtr> {
        vec![self.r#type.ptr_clone()]
    }

    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![(
            self.r#type.ptr_clone(),
            DBuiltin::is_subtype_of(self.r#type.ptr_clone(), DBuiltin::r#type().into_ptr())
                .into_ptr(),
        )]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(self.r#type.ptr_clone())
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

impl DHole {
    pub fn new(r#type: ItemPtr) -> Self {
        Self { r#type }
    }
}
