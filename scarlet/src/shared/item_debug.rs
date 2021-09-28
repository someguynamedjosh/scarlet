use std::fmt::{self, Debug, Formatter};

use super::Item;

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let spacer = if f.alternate() { "\n" } else { " " };
        let nested_spacer = if f.alternate() { "\n    " } else { " " };
        match self {
            Self::Any { selff, typee } => write!(f, "any{{{:?}}} at {:?}", typee, selff),
            Self::BuiltinOperation(bo) => write!(f, "{:?}", bo),
            Self::BuiltinValue(bv) => write!(f, "{:?}", bv),
            Self::Defining { base, definitions } => {
                write!(f, "{:?}{}defining{{", base, spacer)?;
                for (name, def) in definitions {
                    write!(f, "{}{} is {:?}", nested_spacer, name, def)?;
                }
                write!(f, "{}}}", spacer)
            }
            Self::FromType { base, vars } => {
                write!(f, "{:?} From{{", base)?;
                for var in vars {
                    write!(f, " {:?}", var)?;
                }
                write!(f, " }}")
            }
            Self::Pick { clauses, default } => {
                write!(f, "pick{{")?;
                let mut first = true;
                for (condition, value) in clauses {
                    if first {
                        write!(f, "{}if {:?}, {:?}", nested_spacer, condition, value)?;
                    } else {
                        write!(f, "{}elif {:?}, {:?} ", nested_spacer, condition, value)?;
                    }
                    first = false;
                }
                write!(f, "{}else {:?} ", nested_spacer, default)?;
                write!(f, "{}}}", spacer)
            }
            Self::Replacing { base, replacements } => {
                write!(f, "{:?}{}replacing{{", base, spacer)?;
                for (target, value) in replacements {
                    write!(f, "{}{:?} with {:?} ", nested_spacer, target, value)?;
                }
                write!(f, "{}}}", spacer)
            }
            Self::TypeIs {
                base_type_only,
                base,
                typee,
            } => {
                let symbol = if *base_type_only { "bt" } else { ":" };
                write!(f, "{:?} {}{:?}", base, symbol, typee)
            }
            Self::Variant { selff, typee } => {
                write!(f, "variant{{{:?}}} at {:?}", typee, selff)
            }
        }
    }
}
