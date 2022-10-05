use std::{collections::HashMap, fmt};

use super::parameter::ParameterPtr;
use crate::{
    diagnostic::Diagnostic,
    item::{
        query::{
            no_type_check_errors, ChildrenQuery, ParametersQuery, Query, QueryContext,
            TypeCheckQuery, TypeQuery,
        },
        type_hints::TypeHint,
        CycleDetectingDebug, Item, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone)]
pub struct DIdentifier {
    identifier: String,
    item: Option<ItemPtr>,
}

impl CycleDetectingDebug for DIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter, stack: &[*const Item]) -> fmt::Result {
        if let Some(item) = &self.item {
            item.fmt(f, stack)
        } else {
            write!(f, "IDENTIFIER({})", self.identifier)
        }
    }
}

impl ItemDefinition for DIdentifier {
    fn children(&self) -> Vec<ItemPtr> {
        vec![]
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

    fn resolve(&mut self, this: &ItemPtr) -> Result<(), Diagnostic> {
        if let Some(item) = this.lookup_identifier(&self.identifier) {
            self.item = Some(item);
            Ok(())
        } else {
            Err(Diagnostic::new()
                .with_text_error(format!("No identifier \"{}\" in scope.", self.identifier))
                .with_item_error(this))
        }
    }
}

impl DIdentifier {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            item: None,
        }
    }
}
