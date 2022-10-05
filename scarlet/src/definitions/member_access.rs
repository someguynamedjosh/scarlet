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
pub struct DMemberAccess {
    base: ItemPtr,
    member_name: String,
}

impl CycleDetectingDebug for DMemberAccess {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result {
        self.base.fmt(f, stack)?;
        write!(f, ".{}", self.member_name)
    }
}

impl ItemDefinition for DMemberAccess {
    fn collect_children(&self, into: &mut Vec<ItemPtr>) {
        into.push(self.base.ptr_clone())
    }

    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        todo!()
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        todo!()
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

impl DMemberAccess {
    pub fn new(base: ItemPtr, member_name: String) -> Self {
        Self { base, member_name }
    }
}
