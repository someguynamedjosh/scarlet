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
        parameters::Parameters,
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone)]
pub struct DNewValue {
    r#type: ItemPtr,
    fields: Vec<ItemPtr>,
}

impl CycleDetectingDebug for DNewValue {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.r#type.fmt(f, ctx)?;
        write!(f, ".new(\n")?;
        for field in &self.fields {
            write!(f, "   ",)?;
            field.fmt(f, ctx)?;
            write!(f, ",\n")?;
        }
        write!(f, ")")
    }
}

impl ItemDefinition for DNewValue {
    fn children(&self) -> Vec<ItemPtr> {
        self.fields.iter().map(|f| f.ptr_clone()).collect_vec()
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        let mut result = Parameters::new_empty();
        for field in &self.fields {
            result.append(field.query_parameters(ctx));
        }
        result
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(DBuiltin::r#type().into_ptr())
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, this: &ItemPtr, _args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
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
            env.get_language_item("True")?.reduce(&HashMap::new()),
            vec![],
        ))
    }

    pub fn r#false(env: &Environment) -> Result<Self, Diagnostic> {
        Ok(Self::new(
            env.get_language_item("False")?.ptr_clone(),
            vec![],
        ))
    }

    pub fn fields(&self) -> &Vec<ItemPtr> {
        &self.fields
    }

    pub fn get_type(&self) -> &ItemPtr {
        &self.r#type
    }
}
