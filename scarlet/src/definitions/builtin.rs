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
    environment::{Env, ItemId},
    shared::TripleBool,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Builtin {
    IsExactly,
    IsSubtypeOf,
    IfThenElse,
    Union,
    GodType,
}

impl Builtin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::IsExactly => "is_exactly",
            Self::IsSubtypeOf => "is_subtype_of",
            Self::IfThenElse => "if_then_else",
            Self::Union => "Union",
            Self::GodType => "Type",
        }
    }

    pub fn default_arg_names(&self) -> &'static [&'static str] {
        match self {
            Builtin::IsExactly => &["Comparee", "Comparand", "comparee", "comparand"][..],
            Builtin::IsSubtypeOf => &["Subtype", "Supertype"][..],
            Builtin::IfThenElse => &["Result", "condition", "true_result", "false_result"],
            Builtin::Union => &["Subtype0", "Subtype1"],
            Builtin::GodType => &[],
        }
    }
}

#[derive(Clone, Debug)]
pub struct DBuiltin {
    builtin: Builtin,
    args: Vec<ItemId>,
}

impl DBuiltin {
    pub fn new_user_facing(builtin: Builtin, env: &Env) -> Result<Self, Diagnostic> {
        let args = builtin
            .default_arg_names()
            .iter()
            .map(|name| env.get_language_item(name))
            .collect::<Result<_, _>>()?;
        Ok(Self { builtin, args })
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

    pub fn god_type() -> Self {
        Self {
            builtin: Builtin::GodType,
            args: vec![],
        }
    }

    pub fn get_builtin(&self) -> Builtin {
        self.builtin
    }

    pub fn get_args(&self) -> &Vec<ItemId> {
        &self.args
    }

    pub fn add_type_asserts(&self, env: &mut Env) {
        match self.builtin {
            Builtin::IsExactly => {
                let god_type = env.god_type();
                let comparee_type = self.args[0];
                let comparand_type = self.args[1];
                let comparee = self.args[2];
                let comparand = self.args[3];
                env.assert_of_type(comparee_type, god_type);
                env.assert_of_type(comparand_type, god_type);
                env.assert_of_type(comparee, comparee_type);
                env.assert_of_type(comparand, comparand_type);
            }
            Builtin::IsSubtypeOf => {
                let god_type = env.god_type();
                let subtype = self.args[0];
                let supertype = self.args[1];
                env.assert_of_type(subtype, god_type);
                env.assert_of_type(supertype, god_type);
            }
            Builtin::IfThenElse => {
                let god_type = env.god_type();
                let bool_type = env.get_language_item("Bool").unwrap();
                let result_type = self.args[0];
                let condition = self.args[1];
                let true_result = self.args[2];
                let false_result = self.args[3];
                env.assert_of_type(result_type, god_type);
                env.assert_of_type(condition, bool_type);
                env.assert_of_type(true_result, result_type);
                env.assert_of_type(false_result, result_type);
            }
            Builtin::Union => {
                let god_type = env.god_type();
                let subtype_0 = self.args[0];
                let subtype_1 = self.args[1];
                env.assert_of_type(subtype_0, god_type);
                env.assert_of_type(subtype_1, god_type);
            }
            Builtin::GodType => {}
        }
    }
}
