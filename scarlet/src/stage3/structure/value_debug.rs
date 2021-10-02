use std::fmt::{self, Debug, Formatter};

use super::Value;

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let spacer = if f.alternate() { "\n" } else { " " };
        let nested_spacer = if f.alternate() { "\n    " } else { " " };
        match self {
            Self::Any { variable } => variable.fmt(f),
            Self::BuiltinOperation(operation) => operation.fmt(f),
            Self::BuiltinValue(value) => write!(f, "{:?}", value),
            Self::From { base, variables } => {
                write!(f, "{:?}{}FromValues{{", base, spacer)?;
                for variable in variables {
                    write!(f, "{}{:?}", nested_spacer, variable,)?;
                }
                write!(f, "{}}}", spacer)
            }
            Self::Replacing { base, replacements } => {
                write!(f, "{:?}{}replacing{{", base, spacer)?;
                for (target, value) in replacements {
                    write!(f, "{}{:?} with {:?} ", nested_spacer, target, value)?;
                }
                write!(f, "{}}}", spacer)
            }
            Self::Variant { variant } => variant.fmt(f),
        }
    }
}
