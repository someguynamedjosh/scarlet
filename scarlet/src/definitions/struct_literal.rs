use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;

use super::{
    builtin::DBuiltin,
    hole::DHole,
    new_type::DNewType,
    parameter::{DParameter, ParameterPtr},
};
use crate::{
    diagnostic::Position,
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
pub struct DStructLiteral {
    fields: Vec<(String, ItemPtr)>,
    /// If true, a type is automatically generated based on the contents. If
    /// false, the type should be inferred.
    is_module: bool,
}

impl CycleDetectingDebug for DStructLiteral {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result {
        write!(f, "[\n")?;
        for field in &self.fields {
            write!(
                f,
                "   {} IS {}",
                field.0,
                field.1.to_indented_string(stack, 2)
            )?;
            write!(f, ",\n")?;
        }
        write!(f, "]")
    }
}

impl ItemDefinition for DStructLiteral {
    fn collect_children(&self, into: &mut Vec<ItemPtr>) {
        for (_, val) in &self.fields {
            val.collect_self_and_children(into);
        }
    }

    fn collect_type_hints(&self, this: &ItemPtr) -> Vec<(ItemPtr, TypeHint)> {
        self.fields
            .iter()
            .map(|(name, value)| {
                (
                    this.ptr_clone(),
                    TypeHint::MustHaveField {
                        name: name.clone(),
                        value: value.ptr_clone(),
                    },
                )
            })
            .collect_vec()
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        if self.is_module {
            let mut fields = Vec::new();
            for (name, value) in &self.fields {
                let r#type = value.query_type(ctx)?;
                fields.push((
                    name.clone(),
                    DParameter::new(128, Position::placeholder(), r#type).into_ptr(),
                ));
            }
            Some(DNewType::new(fields).into_ptr())
        } else {
            Some(DHole::new(DBuiltin::r#type().into_ptr()).into_ptr())
        }
    }

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        if self.is_module {
            no_type_check_errors()
        } else {
            todo!()
        }
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> Option<ItemPtr> {
        let fields = self
            .fields
            .iter()
            .map(|(name, value)| value.reduce(args).map(|v| (name.clone(), v)))
            .collect::<Option<_>>()?;
        if fields == self.fields {
            Some(this.ptr_clone())
        } else {
            Some(
                Self {
                    fields,
                    is_module: self.is_module,
                }
                .into_ptr(),
            )
        }
    }
}

impl DStructLiteral {
    pub fn new_module(fields: Vec<(String, ItemPtr)>) -> Self {
        Self {
            fields,
            is_module: true,
        }
    }

    pub fn new_struct(fields: Vec<(String, ItemPtr)>) -> Self {
        Self {
            fields,
            is_module: false,
        }
    }
}
