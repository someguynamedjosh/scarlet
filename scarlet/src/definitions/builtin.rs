use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::parameter::ParameterPtr;
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{
        query::{
            no_type_check_errors, ChildrenQuery, ParametersQuery, Query, QueryContext,
            TypeCheckQuery, TypeQuery,
        },
        type_hints::TypeHint,
        CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
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
}

#[derive(Clone)]
pub struct DBuiltin {
    builtin: Builtin,
    args: Vec<ItemPtr>,
}

impl CycleDetectingDebug for DBuiltin {
    fn fmt(&self, f: &mut Formatter, _stack: &[*const Item]) -> fmt::Result {
        write!(f, "BUILTIN({})", self.builtin.name())
    }
}

impl ItemDefinition for DBuiltin {
    fn collect_children(&self, into: &mut Vec<ItemPtr>) {}

    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(Self::r#type().into_ptr())
    }

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        this.ptr_clone()
    }
}

impl DBuiltin {
    pub fn new_user_facing(builtin: Builtin, env: &Environment) -> Result<Self, Diagnostic> {
        let arg_names = match builtin {
            Builtin::IsExactly => &["comparee", "comparand"][..],
            Builtin::IsSubtypeOf => &["Subtype", "Supertype"][..],
            Builtin::IfThenElse => &["condition", "result_when_true", "result_when_false"],
            Builtin::Type => &[],
            Builtin::Union => &["Subtype0", "Subtype1"],
        };
        let args = arg_names
            .iter()
            .map(|name| env.get_language_item(name).map(ItemPtr::ptr_clone))
            .collect::<Result<_, _>>()?;
        Ok(Self { builtin, args })
    }

    pub fn r#type() -> Self {
        Self {
            builtin: Builtin::Type,
            args: vec![],
        }
    }

    pub fn is_type(subtype: ItemPtr) -> Self {
        Self::is_subtype_of(subtype, Self::r#type().into_ptr())
    }

    pub fn is_subtype_of(subtype: ItemPtr, supertype: ItemPtr) -> Self {
        Self {
            builtin: Builtin::Union,
            args: vec![subtype, supertype],
        }
    }

    pub fn union(subtype_0: ItemPtr, subtype_1: ItemPtr) -> Self {
        Self {
            builtin: Builtin::Union,
            args: vec![subtype_0, subtype_1],
        }
    }
}
