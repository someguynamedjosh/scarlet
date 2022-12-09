use std::{collections::HashMap, fmt};

use super::parameter::ParameterPtr;
use crate::{
    diagnostic::Diagnostic,
    item::{
        parameters::Parameters,
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, ItemDefinition, ItemPtr, LazyItemPtr,
    },
};

#[derive(Clone, Debug)]
pub struct DReference {
    target: LazyItemPtr,
}

impl CycleDetectingDebug for DReference {
    fn fmt(&self, f: &mut fmt::Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.target.fmt(f, ctx)
    }
}

impl ItemDefinition for DReference {
    fn children(&self) -> Vec<LazyItemPtr> {
        vec![]
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(LazyItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        self.target.evaluate().unwrap().query_parameters(ctx)
    }

    fn recompute_type(&self, ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        self.target.evaluate().unwrap().query_type(ctx)
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, LazyItemPtr>) -> ItemPtr {
        self.target
            .evaluate()
            .unwrap()
            .reduced(args.clone())
            .evaluate()
            .unwrap()
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<crate::item::query::ResolveQuery>,
    ) -> <crate::item::query::ResolveQuery as Query>::Result {
        Ok(this.ptr_clone())
    }
}

impl DReference {
    pub fn new(target: LazyItemPtr) -> Self {
        Self { target }
    }
}
