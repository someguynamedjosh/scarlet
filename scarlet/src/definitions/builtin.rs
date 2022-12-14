#[cfg(not(feature = "trace_borrows"))]
use std::cell::{Ref, RefCell};
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

#[cfg(feature = "trace_borrows")]
use debug_cell::{Ref, RefCell, RefMut};
use itertools::Itertools;
use owning_ref::OwningRef;

use super::{compound_type::DCompoundType, parameter::ParameterPtr};
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{CddContext, CycleDetectingDebug, Item, ItemDefinition, ItemRef},
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

#[derive(Clone)]
pub struct DBuiltin<Definition, Analysis> {
    builtin: Builtin,
    args: Vec<ItemRef<Definition, Analysis>>,
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> CycleDetectingDebug
    for DBuiltin<Definition, Analysis>
{
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "BUILTIN({})", self.builtin.name())?;
        if self.args.len() == 0 {
            return Ok(());
        }
        write!(f, "(\n")?;
        for (param_name, arg) in self
            .builtin
            .default_arg_names()
            .iter()
            .zip(self.args.iter())
        {
            write!(f, "   {} IS {}", param_name, arg.to_indented_string(ctx, 2))?;
            write!(f, ",\n")?;
        }
        write!(f, ")")
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DBuiltin<Definition, Analysis>
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        self.args.iter().map(ItemRef::ptr_clone).collect()
    }
}

impl<Definition, Analysis> DBuiltin<Definition, Analysis> {
    pub fn new_user_facing(
        builtin: Builtin,
        env: &Environment<Definition, Analysis>,
    ) -> Result<Self, Diagnostic> {
        todo!()
    }

    pub fn is_type(candidate: ItemRef<Definition, Analysis>) -> Self {
        todo!()
    }

    pub fn is_subtype_of(
        subtype: ItemRef<Definition, Analysis>,
        supertype: ItemRef<Definition, Analysis>,
    ) -> Self {
        Self {
            builtin: Builtin::IsSubtypeOf,
            args: vec![subtype, supertype],
        }
    }

    pub fn union(
        subtype_0: ItemRef<Definition, Analysis>,
        subtype_1: ItemRef<Definition, Analysis>,
    ) -> Self {
        Self {
            builtin: Builtin::Union,
            args: vec![subtype_0, subtype_1],
        }
    }

    pub fn get_builtin(&self) -> Builtin {
        self.builtin
    }

    pub fn get_args(&self) -> &Vec<ItemRef<Definition, Analysis>> {
        &self.args
    }
}
