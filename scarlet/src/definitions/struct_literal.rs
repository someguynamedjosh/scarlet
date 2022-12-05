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
        parameters::Parameters, CddContext, CycleDetectingDebug, ItemEnum, ItemPtr, LazyItemPtr,
    },
};

#[derive(Clone)]
pub struct DStructLiteral<I: ItemEnum> {
    fields: Vec<(String, LazyItemPtr<I>)>,
    /// If true, a type is automatically generated based on the contents. If
    /// false, the type should be inferred.
    is_module: bool,
}

impl<I: ItemEnum> CycleDetectingDebug for DStructLiteral<I> {
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

impl<I: ItemEnum> DStructLiteral<I> {
    pub fn new_module(fields: Vec<(String, LazyItemPtr<I>)>) -> Self {
        Self {
            fields,
            is_module: true,
        }
    }

    pub fn new_struct(fields: Vec<(String, LazyItemPtr<I>)>) -> Self {
        Self {
            fields,
            is_module: false,
        }
    }

    pub fn is_module(&self) -> bool {
        self.is_module
    }
}
