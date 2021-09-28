use std::fmt::{self, Debug, Formatter};

use super::Item;

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let spacer = if f.alternate() { "\n" } else { " " };
        let nested_spacer = if f.alternate() { "\n    " } else { " " };
        match self {
            Self::Defining { base, definitions } => {
                write!(f, "{:?}{}defining{{", base, spacer)?;
                for (name, def) in definitions {
                    write!(f, "{}{} is {:?}", nested_spacer, name, def)?;
                }
                write!(f, "{}}}", spacer)
            }
            Self::FromType { base, values: vars } => {
                write!(f, "{:?} From{{", base)?;
                for var in vars {
                    write!(f, " {:?}", var)?;
                }
                write!(f, " }}")
            }
            Self::GodType => write!(f, "TYPE"),
            Self::VariantInstance {
                typee,
                variant_id,
                values: params,
            } => {
                write!(f, "value{{{:?} {:?} [", typee, variant_id)?;
                for param in params {
                    write!(f, "{}{:?}, ", nested_spacer, param)?;
                }
                write!(f, "{}]}}", spacer)
            }
            Self::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                write!(f, "pick{{")?;
                let (condition, value) = initial_clause;
                write!(f, "{}if {:?}, {:?}", nested_spacer, condition, value)?;
                for (condition, value) in elif_clauses {
                    write!(f, "{}elif {:?}, {:?} ", nested_spacer, condition, value)?;
                }
                write!(f, "{}else {:?} ", nested_spacer, else_clause)?;
                write!(f, "{}}}", spacer)
            }
            Self::BuiltinOperation(po) => write!(f, "{:?}", po),
            Self::PrimitiveType(pt) => write!(f, "{:?}", pt),
            Self::PrimitiveValue(pv) => write!(f, "{:?}", pv),
            Self::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => {
                write!(f, "{:?}{}{:#?}", base, spacer, replacements)?;
                for value in unlabeled_replacements {
                    write!(f, "{}{:?}", nested_spacer, value)?;
                }
                for (target, value) in replacements {
                    write!(f, "{}{:?} with {:?} ", nested_spacer, target, value)?;
                }
                write!(f, "{}}}", spacer)
            }
            Self::TypeIs { exact, base, typee } => {
                let open = if *exact { "type_is_exactly{" } else { ":" };
                let close = if *exact { "}" } else { "" };
                write!(f, "{:?} {}{:?}{}", base, open, typee, close)
            }
            Self::Variable { selff, typee } => write!(f, "any{{{:?}}} at {:?}", typee, selff),
        }
    }
}
