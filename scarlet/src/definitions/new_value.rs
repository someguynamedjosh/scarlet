use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
};

use itertools::Itertools;

use super::{
    compound_type::{DCompoundType, Type},
    parameter::ParameterPtr,
};
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{
        parameters::Parameters,
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, ResolveQuery,
            TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr, LazyItemPtr,
    },
    util::PtrExtension,
};

#[derive(Clone)]
pub struct DNewValue {
    r#type: Rc<Type>,
    fields: Vec<LazyItemPtr>,
}

impl CycleDetectingDebug for DNewValue {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.r#type.fmt(f, ctx)?;
        write!(f, ".new(\n")?;
        for field in &self.fields {
            write!(f, "   {},\n", field.to_indented_string(ctx, 1))?;
        }
        write!(f, ")")
    }
}

impl ItemDefinition for DNewValue {
    fn children(&self) -> Vec<LazyItemPtr> {
        self.fields.iter().map(|f| f.ptr_clone()).collect_vec()
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(LazyItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        let mut result = Parameters::new_empty();
        for field in &self.fields {
            let field = field.evaluate().unwrap();
            result.append(field.query_parameters(ctx));
        }
        result
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(
            DCompoundType::new_single(self.r#type.ptr_clone())
                .into_ptr()
                .into_lazy(),
        )
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
        let rfields = self
            .fields
            .iter()
            .map(|field| field.evaluate().unwrap().resolved())
            .collect();
        if rfields == self.fields {
            Ok(this.ptr_clone())
        } else {
            Ok(Self {
                fields: rfields,
                r#type: self.r#type.ptr_clone(),
            }
            .into_ptr_mimicking(this))
        }
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, LazyItemPtr>) -> ItemPtr {
        let rfields = self
            .fields
            .iter()
            .map(|field| field.evaluate().unwrap().reduced(args.clone()))
            .collect_vec();
        if rfields == self.fields {
            this.ptr_clone()
        } else {
            Self {
                fields: rfields,
                r#type: self.r#type.ptr_clone(),
            }
            .into_ptr_mimicking(this)
        }
    }
}

impl DNewValue {
    pub fn new(r#type: Rc<Type>, fields: Vec<LazyItemPtr>) -> Self {
        assert!(!r#type.is_god_type());
        assert_eq!(r#type.get_fields().len(), fields.len());
        Self { r#type, fields }
    }

    fn get_builtin_type(env: &Environment, name: &str) -> Result<Rc<Type>, Diagnostic> {
        Ok(env
            .get_language_item(name)?
            .resolved()
            .evaluate()
            .unwrap()
            .downcast_definition::<DCompoundType>()
            .unwrap()
            .as_ref()
            .get_component_types()
            .iter()
            .next()
            .unwrap()
            .1
            .ptr_clone())
    }

    pub fn r#true(env: &Environment) -> Result<Self, Diagnostic> {
        Ok(Self::new(Self::get_builtin_type(env, "True")?, vec![]))
    }

    pub fn r#false(env: &Environment) -> Result<Self, Diagnostic> {
        Ok(Self::new(Self::get_builtin_type(env, "False")?, vec![]))
    }

    pub fn fields(&self) -> &Vec<LazyItemPtr> {
        &self.fields
    }

    pub fn get_type(&self) -> &Rc<Type> {
        &self.r#type
    }
}
