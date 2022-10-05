use std::{collections::HashMap, fmt};

use super::parameter::ParameterPtr;
use crate::item::{
    query::{
        no_type_check_errors, ChildrenQuery, ParametersQuery, Query, QueryContext, TypeCheckQuery,
        TypeQuery,
    },
    type_hints::TypeHint,
    CycleDetectingDebug, Item, ItemDefinition, ItemPtr,
};

#[derive(Clone)]
pub struct DIdentifier {
    identifier: String,
}

impl CycleDetectingDebug for DIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter, _stack: &[*const Item]) -> fmt::Result {
        write!(f, "ident\"{}\"", self.identifier)
    }
}

impl ItemDefinition for DIdentifier {
    fn collect_children(&self, into: &mut Vec<ItemPtr>) {}

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
        None
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

impl DIdentifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}
