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
        CddContext, CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
    },
    scope::Scope,
};

#[derive(Clone)]
pub struct DStructLiteral {
    fields: Vec<(String, ItemPtr)>,
    /// If true, a type is automatically generated based on the contents. If
    /// false, the type should be inferred.
    is_module: bool,
}

impl CycleDetectingDebug for DStructLiteral {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "[\n")?;
        for field in &self.fields {
            write!(
                f,
                "   {} IS {}",
                field.0,
                field.1.to_indented_string(ctx, 2)
            )?;
            write!(f, ",\n")?;
        }
        write!(f, "]")
    }
}

impl ItemDefinition for DStructLiteral {
    fn children(&self) -> Vec<ItemPtr> {
        self.fields.iter().map(|(_, f)| f.ptr_clone()).collect_vec()
    }

    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        if self.is_module {
            vec![]
        } else {
            todo!()
        }
    }

    fn local_lookup_identifier(&self, identifier: &str) -> Option<ItemPtr> {
        for (field, value) in &self.fields {
            if field == identifier {
                return Some(value.ptr_clone());
            }
        }
        None
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

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        let fields = self
            .fields
            .iter()
            .map(|(name, value)| (name.clone(), value.reduce(args)))
            .collect();
        if fields == self.fields {
            this.ptr_clone()
        } else {
            Self {
                fields,
                is_module: self.is_module,
            }
            .into_ptr()
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
