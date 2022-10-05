use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;

use super::{builtin::DBuiltin, new_type::DNewType, parameter::ParameterPtr};
use crate::{
    diagnostic::Diagnostic,
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
pub struct DNewValue {
    r#type: ItemPtr,
    fields: Vec<ItemPtr>,
}

impl CycleDetectingDebug for DNewValue {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result {
        self.r#type.fmt(f, stack);
        write!(f, ".new(\n")?;
        for field in &self.fields {
            write!(f, "   ",)?;
            field.fmt(f, stack)?;
            write!(f, ",\n")?;
        }
        write!(f, ")")
    }
}

impl ItemDefinition for DNewValue {
    fn collect_children(&self, into: &mut Vec<ItemPtr>) {
        for field in &self.fields {
            field.collect_self_and_children(into);
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

impl DNewValue {
    pub fn new(r#type: ItemPtr, fields: Vec<ItemPtr>) -> Self {
        if let Some(new_type) = r#type.downcast_definition::<DNewType>() {
            assert_eq!(new_type.get_fields().len(), fields.len())
        }
        Self { r#type, fields }
    }

    pub fn r#true(env: &Environment) -> Result<Self, Diagnostic> {
        Ok(Self::new(
            env.get_language_item("True")?.ptr_clone(),
            vec![],
        ))
    }

    pub fn r#false(env: &Environment) -> Result<Self, Diagnostic> {
        Ok(Self::new(
            env.get_language_item("False")?.ptr_clone(),
            vec![],
        ))
    }

    pub fn get_type(&self) -> &ItemPtr {
        &self.r#type
    }
}
