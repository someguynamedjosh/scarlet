#[cfg(not(feature = "trace_borrows"))]
use std::cell::{Ref, RefCell};
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

#[cfg(feature = "trace_borrows")]
use debug_cell::{Id, IdCell, IdMut};
use itertools::Itertools;
use owning_ref::OwningRef;

use super::compound_type::DCompoundType;
use crate::{
    diagnostic::Diagnostic,
    environment::{Environment, ItemId, ENV},
    shared::TripleBool,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Builtin {
    IsExactly,
    IsSubtypeOf,
    IfThenElse,
    Union,
}

impl Builtin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::IsExactly => "is_exactly",
            Self::IsSubtypeOf => "is_subtype_of",
            Self::IfThenElse => "if_then_else",
            Self::Union => "Union",
        }
    }

    pub fn default_arg_names(&self) -> &'static [&'static str] {
        match self {
            Builtin::IsExactly => &["Comparee", "Comparand", "comparee", "comparand"][..],
            Builtin::IsSubtypeOf => &["Subtype", "Supertype"][..],
            Builtin::IfThenElse => &["Result", "condition", "true_result", "false_result"],
            Builtin::Union => &["Subtype0", "Subtype1"],
        }
    }
}

#[derive(Clone, Debug)]
pub struct DBuiltin {
    builtin: Builtin,
    args: Vec<ItemId>,
}

impl DBuiltin {
    pub fn new_user_facing<I>(builtin: Builtin, env: &Environment<I>) -> Result<Self, Diagnostic> {
        let args = builtin
            .default_arg_names()
            .iter()
            .map(|name| env.get_language_item(name))
            .collect::<Result<_, _>>()?;
        Ok(Self { builtin, args })
    }

    pub fn is_type(candidate: ItemId) -> Self {
        todo!()
    }

    pub fn is_subtype_of(subtype: ItemId, supertype: ItemId) -> Self {
        Self {
            builtin: Builtin::IsSubtypeOf,
            args: vec![subtype, supertype],
        }
    }

    pub fn union(subtype_0: ItemId, subtype_1: ItemId) -> Self {
        Self {
            builtin: Builtin::Union,
            args: vec![subtype_0, subtype_1],
        }
    }

    pub fn get_builtin(&self) -> Builtin {
        self.builtin
    }

    pub fn get_args(&self) -> &Vec<ItemId> {
        &self.args
    }
}
