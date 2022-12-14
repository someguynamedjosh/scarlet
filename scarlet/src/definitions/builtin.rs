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
    environment::{r#false, r#true, Environment, ENV},
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

#[derive(Clone, Debug)]
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

fn both_compound_types<'a, Definition, Analysis>(
    a: &'a ItemRef<Definition, Analysis>,
    b: &'a ItemRef<Definition, Analysis>,
) -> Option<(
    OwningRef<Ref<'a, Item<Definition, Analysis>>, DCompoundType<Definition, Analysis>>,
    OwningRef<Ref<'a, Item<Definition, Analysis>>, DCompoundType<Definition, Analysis>>,
)> {
    a.downcast_definition::<DCompoundType>()
        .map(|def_a| {
            b.downcast_definition::<DCompoundType>()
                .map(|def_b| (def_a, def_b))
        })
        .flatten()
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DBuiltin<Definition, Analysis>
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        self.args.iter().map(ItemRef::ptr_clone).collect()
    }
}

impl<Definition, Analysis> DBuiltin<Definition, Analysis> {
    pub fn new_user_facing(builtin: Builtin, env: &Environment) -> Result<Self, Diagnostic> {
        let args = builtin
            .default_arg_names()
            .iter()
            .map(|name| env.get_language_item(name).map(ItemRef::ptr_clone))
            .collect::<Result<_, _>>()?;
        let _true = Some(env.r#true());
        Ok(Self { builtin, args })
    }

    pub fn is_type(candidate: ItemRef<Definition, Analysis>) -> Self {
        Self::is_subtype_of(
            candidate
                .query_type(&mut Environment::root_query())
                .unwrap(),
            DCompoundType::r#type().into_ptr(),
        )
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
