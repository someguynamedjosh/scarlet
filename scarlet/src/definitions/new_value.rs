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
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
    },
    shared::TripleBool,
    util::PtrExtension,
};

#[derive(Clone)]
pub struct DNewValue {
    r#type: Rc<Type>,
    type_expr: ItemPtr,
    fields: Vec<ItemPtr>,
}

impl CycleDetectingDebug for DNewValue {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.type_expr.fmt(f, ctx)?;
        write!(f, ".new(\n")?;
        for field in &self.fields {
            write!(f, "   {},\n", field.to_indented_string(ctx, 1))?;
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
        Some(DCompoundType::new_single(self.r#type.ptr_clone()).into_ptr())
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
        let rfields = self.fields.iter().map(|field| field.resolved()).collect();
        if rfields == self.fields {
            Ok(this.ptr_clone())
        } else {
            Ok(Self {
                fields: rfields,
                r#type: self.r#type.ptr_clone(),
                type_expr: self.type_expr.ptr_clone(),
            }
            .into_ptr_mimicking(this))
        }
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        let rfields = self
            .fields
            .iter()
            .map(|field| field.reduced(args, true))
            .collect_vec();
        Self {
            fields: rfields,
            r#type: self.r#type.ptr_clone(),
            type_expr: self.type_expr.ptr_clone(),
        }
        .into_ptr_mimicking(this)
    }

    fn is_equal_to(&self, other: &ItemPtr) -> TripleBool {
        if let Some(other) = other.dereference().unwrap().downcast_definition::<Self>() {
            if !self.r#type.is_same_type_as(&other.r#type) {
                return TripleBool::False;
            }
            for (lfield, rfield) in self.fields.iter().zip(other.fields.iter()) {
                let fields_equal = lfield.is_equal_to(&rfield);
                if fields_equal != TripleBool::True {
                    return fields_equal;
                }
            }
            TripleBool::True
        } else {
            TripleBool::Unknown
        }
    }
}

impl DNewValue {
    pub fn new(r#type: Rc<Type>, type_expr: ItemPtr, fields: Vec<ItemPtr>) -> Self {
        assert!(!r#type.is_god_type());
        assert_eq!(r#type.get_fields().len(), fields.len());
        Self {
            r#type,
            type_expr,
            fields,
        }
    }

    fn get_builtin_type(env: &Environment, name: &str) -> Result<(Rc<Type>, ItemPtr), Diagnostic> {
        let expr = env
            .get_language_item(name)?
            .resolved()
            .dereference()
            .unwrap();

        let r#type = expr
            .downcast_definition::<DCompoundType>()
            .unwrap()
            .as_ref()
            .get_component_types()
            .iter()
            .next()
            .unwrap()
            .1
            .ptr_clone();

        Ok((r#type, expr))
    }

    pub fn r#true(env: &Environment) -> Result<Self, Diagnostic> {
        let (r#type, expr) = Self::get_builtin_type(env, "True")?;
        Ok(Self::new(r#type, expr, vec![]))
    }

    pub fn r#false(env: &Environment) -> Result<Self, Diagnostic> {
        let (r#type, expr) = Self::get_builtin_type(env, "False")?;
        Ok(Self::new(r#type, expr, vec![]))
    }

    pub fn fields(&self) -> &Vec<ItemPtr> {
        &self.fields
    }

    pub fn get_type(&self) -> &Rc<Type> {
        &self.r#type
    }
}
