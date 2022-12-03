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
        parameters::Parameters,
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, ResolveQuery,
            TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
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

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
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

    fn local_reverse_lookup_identifier(&self, item: &ItemPtr) -> Option<String> {
        for (field, value) in &self.fields {
            if value.is_same_instance_as(item) {
                return Some(field.clone());
            }
        }
        None
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        let mut result = Parameters::new_empty();
        if self.is_module {
            return result;
        }
        for field in &self.fields {
            result.append(field.1.query_parameters(ctx));
        }
        result
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
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        if self.is_module {
            no_type_check_errors()
        } else {
            todo!()
        }
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        let fields = self
            .fields
            .iter()
            .map(|(name, value)| value.query_resolved(ctx).map(|value| (name.clone(), value)))
            .try_collect()?;
        Ok(if fields == self.fields {
            this.ptr_clone()
        } else {
            Self {
                fields,
                is_module: self.is_module,
            }
            .into_ptr_mimicking(this)
        })
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
            .into_ptr_mimicking(this)
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

    pub fn is_module(&self) -> bool {
        self.is_module
    }
}
