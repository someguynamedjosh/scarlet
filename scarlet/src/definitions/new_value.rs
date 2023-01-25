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
    item::{CddContext, CycleDetectingDebug, ItemDefinition, ItemRef},
    shared::TripleBool,
    util::PtrExtension,
};

pub struct DNewValue<Definition, Analysis> {
    r#type: Rc<Type<Definition, Analysis>>,
    type_expr: ItemRef<Definition, Analysis>,
    fields: Vec<ItemRef<Definition, Analysis>>,
}

impl<Definition, Analysis> Clone for DNewValue<Definition, Analysis> {
    fn clone(&self) -> Self {
        Self {
            r#type: self.r#type.ptr_clone(),
            type_expr: self.type_expr.ptr_clone(),
            fields: self.fields.clone(),
        }
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> CycleDetectingDebug
    for DNewValue<Definition, Analysis>
{
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.type_expr.fmt(f, ctx)?;
        write!(f, ".new(\n")?;
        for field in &self.fields {
            write!(f, "   {},\n", field.to_indented_string(ctx, 1))?;
        }
        write!(f, ")")
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DNewValue<Definition, Analysis>
{
    fn map_children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        self.fields.iter().map(|f| f.ptr_clone()).collect_vec()
    }
}

impl<Definition, Analysis> DNewValue<Definition, Analysis> {
    pub fn new(
        r#type: Rc<Type<Definition, Analysis>>,
        type_expr: ItemRef<Definition, Analysis>,
        fields: Vec<ItemRef<Definition, Analysis>>,
    ) -> Self {
        assert!(!r#type.is_god_type());
        assert_eq!(r#type.get_fields().len(), fields.len());
        Self {
            r#type,
            type_expr,
            fields,
        }
    }

    fn get_builtin_type(
        env: &Environment<Definition, Analysis>,
        name: &str,
    ) -> Result<
        (
            Rc<Type<Definition, Analysis>>,
            ItemRef<Definition, Analysis>,
        ),
        Diagnostic,
    > {
        todo!()
    }

    pub fn r#true(env: &Environment<Definition, Analysis>) -> Result<Self, Diagnostic> {
        let (r#type, expr) = Self::get_builtin_type(env, "True")?;
        Ok(Self::new(r#type, expr, vec![]))
    }

    pub fn r#false(env: &Environment<Definition, Analysis>) -> Result<Self, Diagnostic> {
        let (r#type, expr) = Self::get_builtin_type(env, "False")?;
        Ok(Self::new(r#type, expr, vec![]))
    }

    pub fn fields(&self) -> &Vec<ItemRef<Definition, Analysis>> {
        &self.fields
    }

    pub fn get_type(&self) -> &Rc<Type<Definition, Analysis>> {
        &self.r#type
    }
}
