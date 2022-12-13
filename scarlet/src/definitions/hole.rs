use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{builtin::DBuiltin, parameter::ParameterPtr, compound_type::DCompoundType};
use crate::item::{
    query::{
        no_type_check_errors, ParametersQuery, Query, QueryContext, ResolveQuery, TypeCheckQuery,
        TypeQuery,
    },
    CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr, 
};

#[derive(Clone)]
pub struct DHole {
    r#type: ItemPtr,
}

impl CycleDetectingDebug for DHole {
    fn fmt(&self, f: &mut Formatter, _ctx: &mut CddContext) -> fmt::Result {
        write!(f, "_")
    }
}

impl ItemDefinition for DHole {
    fn children(&self) -> Vec<ItemPtr> {
        vec![self.r#type.ptr_clone()]
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![(
            self.r#type.ptr_clone(),
            DBuiltin::is_subtype_of(
                self.r#type.ptr_clone(),
                DCompoundType::r#type().into_ptr(),
            )
            .into_ptr(),
        )]
    }

    fn recompute_parameters(
        &self,
        _ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(self.r#type.ptr_clone())
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        Ok(this.ptr_clone())
    }

    fn reduce(&self, this: &ItemPtr, _args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        this.ptr_clone()
    }
}

impl DHole {
    pub fn new(r#type: ItemPtr) -> Self {
        Self { r#type }
    }
}
