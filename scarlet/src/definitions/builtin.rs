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
    item::{
        parameters::Parameters, CddContext, CycleDetectingDebug, Item, ItemEnum, ItemPtr,
        LazyItemPtr,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Builtin {
    IsExactly,
    IsSubtypeOf,
    IfThenElse,
    Type,
    Union,
}

impl Builtin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::IsExactly => "is_exactly",
            Self::IsSubtypeOf => "is_subtype_of",
            Self::IfThenElse => "if_then_else",
            Self::Type => "Type",
            Self::Union => "Union",
        }
    }

    pub fn default_arg_names(&self) -> &'static [&'static str] {
        match self {
            Builtin::IsExactly => &["comparee", "comparand"][..],
            Builtin::IsSubtypeOf => &["Subtype", "Supertype"][..],
            Builtin::IfThenElse => &["Result", "condition", "true_result", "false_result"],
            Builtin::Type => &[],
            Builtin::Union => &["Subtype0", "Subtype1"],
        }
    }
}

#[derive(Clone, Debug)]
pub struct DBuiltin<I: ItemEnum> {
    builtin: Builtin,
    args: Vec<LazyItemPtr<I>>,
}

impl<I: ItemEnum> CycleDetectingDebug for DBuiltin<I> {
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

fn both_compound_types<'a, I: ItemEnum>(
    a: &'a ItemPtr<I>,
    b: &'a ItemPtr<I>,
) -> Option<(
    OwningRef<Ref<'a, Item<I>>, DCompoundType>,
    OwningRef<Ref<'a, Item<I>>, DCompoundType>,
)> {
    a.downcast_definition::<DCompoundType>()
        .map(|def_a| {
            b.downcast_definition::<DCompoundType>()
                .map(|def_b| (def_a, def_b))
        })
        .flatten()
}

impl<I: ItemEnum> DBuiltin<I> {
    pub fn new_user_facing(builtin: Builtin, env: &Environment) -> Result<Self, Diagnostic> {
        let args = builtin
            .default_arg_names()
            .iter()
            .map(|name| {
                env.get_language_item(name)
                    .map(ItemPtr::ptr_clone)
                    .map(ItemPtr::into_lazy)
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { builtin, args })
    }

    pub fn r#type() -> Self {
        Self {
            builtin: Builtin::Type,
            args: vec![],
        }
    }

    pub fn is_type(candidate: ItemPtr<I>) -> Self {
        Self::is_subtype_of(
            candidate
                .query_type(&mut Environment::root_query())
                .unwrap(),
            Self::r#type().into_ptr().into_lazy(),
        )
    }

    pub fn is_subtype_of(subtype: LazyItemPtr<I>, supertype: LazyItemPtr<I>) -> Self {
        Self {
            builtin: Builtin::IsSubtypeOf,
            args: vec![subtype, supertype],
        }
    }

    pub fn union(subtype_0: LazyItemPtr<I>, subtype_1: LazyItemPtr<I>) -> Self {
        Self {
            builtin: Builtin::Union,
            args: vec![subtype_0, subtype_1],
        }
    }

    pub fn get_builtin(&self) -> Builtin {
        self.builtin
    }

    pub fn get_args(&self) -> &Vec<LazyItemPtr<I>> {
        &self.args
    }
}
