use std::fmt::{self, Formatter};

use super::builtin::DBuiltin;
use crate::item::{
    query::{
        no_type_check_errors, ChildrenQuery, ParametersQuery, Query, QueryContext, TypeCheckQuery,
        TypeQuery,
    },
    type_hints::TypeHint,
    CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
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
    fn collect_children(&self, into: &mut Vec<ItemPtr>) {
        self.r#type.collect_self_and_children(into)
    }

    fn collect_type_hints(&self, this: &ItemPtr) -> Vec<(ItemPtr, TypeHint)> {
        vec![(
            self.r#type.ptr_clone(),
            TypeHint::MustBeContainedIn(DBuiltin::r#type().into_ptr()),
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
}

impl DHole {
    pub fn new(r#type: ItemPtr) -> Self {
        Self { r#type }
    }
}
