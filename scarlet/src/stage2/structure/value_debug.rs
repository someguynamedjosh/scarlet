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
            Self::From { base, values } => {
                write!(f, "{:?}{}FromItems{{", base, spacer)?;
                for item in values {
                    write!(f, "{}{:?}", nested_spacer, item,)?;
                }
                write!(f, "{}}}", spacer)
            }
            Self::Member {
                base, name: member, ..
            } => write!(f, "{:?}::{}", base, member),
            Self::Identifier { name, in_namespace } => {
                write!(f, "identifier{{{}}} in {:?}", name, in_namespace)
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
