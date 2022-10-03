use std::fmt::{self, Formatter};

use crate::item::{
    query::{
        no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
    },
    CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
};

pub enum Builtin {
    IsExactly,
    IfThenElse,
    Type,
    Union,
}

impl Builtin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::IsExactly => "is_exactly",
            Self::IfThenElse => "if_then_else",
            Self::Type => "Type",
            Self::Union => "Union",
        }
    }
}

#[derive(Clone)]
pub struct DBuiltin {
    builtin: Builtin,
}

impl CycleDetectingDebug for DBuiltin {
    fn fmt(&self, f: &mut Formatter, _stack: &[*const Item]) -> fmt::Result {
        write!(f, "BUILTIN({})", self.builtin.name())
    }
}

impl ItemDefinition for DBuiltin {
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
}

impl DBuiltin {
    pub fn new(builtin: Builtin) -> Self {
        DBuiltin { builtin }
    }

    pub(crate) fn r#type() -> Self {
        Self::new(Builtin::Type)
    }
}
