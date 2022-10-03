use std::fmt;

use crate::item::{
    query::{
        no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
    },
    CycleDetectingDebug, Item, ItemDefinition,
};

#[derive(Clone)]
pub struct DIdentifier {
    identifier: String,
}

impl CycleDetectingDebug for DIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter, _stack: &[*const Item]) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl ItemDefinition for DIdentifier {
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
}

impl DIdentifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}
