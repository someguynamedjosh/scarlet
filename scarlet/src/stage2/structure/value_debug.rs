use std::fmt::{self, Debug, Formatter};

use super::Value;

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let spacer = if f.alternate() { "\n" } else { " " };
        let nested_spacer = if f.alternate() { "\n    " } else { " " };
        match self {
            Self::Any { variable } => variable.fmt(f),
            Self::BuiltinOperation { operation } => operation.fmt(f),
            Self::BuiltinValue { value } => value.fmt(f),
            Self::Defining {
                base,
                definitions,
                this_scope: child_scope,
            } => {
                write!(
                    f,
                    "{:?}{}at {:?}{}defining{{",
                    base, spacer, child_scope, spacer
                )?;
                for (name, def) in definitions {
                    write!(f, "{}{} is {:?}", nested_spacer, name, def)?;
                }
                write!(f, "{}}}", spacer)
            }
            Self::FromItems { base, items } => {
                write!(f, "{:?} FromItems{{", base)?;
                for variable in items {
                    write!(f, " {:?}", variable)?;
                }
                write!(f, " }}")
            }
            Self::FromVariables { base, variables } => {
                write!(f, "{:?} FromVariables{{", base)?;
                for variable in variables {
                    write!(f, " {:?}", variable)?;
                }
                write!(f, " }}")
            }
            Self::Item { item } => item.fmt(f),
            Self::Member { base, member } => write!(f, "{:?}::{}", base, member),
            Self::Identifier { name } => write!(f, "identifier{{{}}}", name),
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
