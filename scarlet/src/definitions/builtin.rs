use std::fmt::{self, Formatter};

use crate::item::{CycleDetectingDebug, Item, ItemDefinition};

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
            Self::Union => "Union"
        }
    }
}

pub struct DBuiltin {
    builtin: Builtin,
}

impl CycleDetectingDebug for DBuiltin {
    fn fmt(&self, f: &mut Formatter, _stack: &[*const Item]) -> fmt::Result {
        write!(f, "BUILTIN({})", self.builtin.name())
    }
}

impl ItemDefinition for DBuiltin {}

impl DBuiltin {
    pub fn new(builtin: Builtin) -> Self {
        DBuiltin { builtin }
    }
}
