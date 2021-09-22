use std::fmt::{self, Debug, Formatter};

use crate::{
    stage2::structure::{
        Definitions, ItemId, PrimitiveOperation, PrimitiveType, PrimitiveValue, Replacements,
    },
    util::indented,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    pub modules: Vec<ItemId>,
    pub(crate) items: Vec<Item>,
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Environment [")?;
        for (index, item) in self.items.iter().enumerate() {
            if f.alternate() {
                write!(f, "\n\n    ")?;
            }
            write!(f, "{:?} is ", ItemId(index))?;
            if f.alternate() {
                let text = format!("{:#?}", item);
                write!(f, "{},", indented(&text[..]))?;
            } else {
                write!(f, "{:?}", item)?;
            }
        }
        if f.alternate() {
            writeln!(f)?;
        }
        write!(f, "]")
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn _mark_as_module(&mut self, item: ItemId) {
        self.modules.push(item)
    }

    pub fn insert(&mut self, definition: Item) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(definition);
        id
    }

    pub fn _definition_of(&self, item: ItemId) -> &Item {
        assert!(item.0 < self.items.len());
        &self.items[item.0]
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Defining {
        base: ItemId,
        definitions: Definitions,
    },
    FromType {
        base: ItemId,
        vars: Vec<ItemId>,
    },
    GodType,
    InductiveType(ItemId),
    InductiveValue {
        typee: ItemId,
        variant_name: String,
        records: Vec<ItemId>,
    },
    IsSameVariant {
        base: ItemId,
        other: ItemId,
    },
    Pick {
        initial_clause: (ItemId, ItemId),
        elif_clauses: Vec<(ItemId, ItemId)>,
        else_clause: ItemId,
    },
    PrimitiveOperation(PrimitiveOperation),
    PrimitiveType(PrimitiveType),
    PrimitiveValue(PrimitiveValue),
    Replacing {
        base: ItemId,
        replacements: Replacements,
        unlabeled_replacements: Vec<ItemId>,
    },
    TypeIs {
        exact: bool,
        base: ItemId,
        typee: ItemId,
    },
    Variable {
        selff: ItemId,
        typee: ItemId,
    },
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Defining { base, definitions } => {
                let gap = if f.alternate() { "\n" } else { "" };
                write!(f, "{:?} {}defining{{", base, gap)?;
                for (name, def) in definitions {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{} is {:?} ", name, def)?;
                }
                write!(f, "{}}}", gap)
            }
            Self::FromType { base, vars } => {
                write!(f, "{:?} From{{", base)?;
                if !vars.is_empty() {
                    write!(f, "{:?}", vars[0])?;
                    for var in &vars[1..] {
                        write!(f, " {:?}", var)?;
                    }
                }
                write!(f, "}}")
            }
            Self::GodType => write!(f, "TYPE"),
            Self::InductiveType(id) => write!(f, "InductiveType({:?})", id),
            Self::InductiveValue {
                typee,
                variant_name,
                records,
            } => {
                write!(f, "inductive_value {:?}::{}[", typee, variant_name)?;
                for record in records {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{:?}, ", record)?;
                }
                if f.alternate() {
                    writeln!(f)?;
                }
                write!(f, "]")
            }
            Self::IsSameVariant { base, other } => {
                write!(f, "{:?} is_same_variant_as{{{:?}}}", base, other)
            }
            Self::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                write!(f, "pick{{ ")?;
                if f.alternate() {
                    write!(f, "\n    ")?;
                }
                write!(f, "if {:?}, {:?} ", initial_clause.0, initial_clause.1)?;
                if f.alternate() {
                    write!(f, "\n    ")?;
                }
                for (condition, value) in elif_clauses {
                    write!(f, "elif {:?}, {:?} ", condition, value)?;
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                }
                write!(f, "else {:?} ", else_clause)?;
                write!(f, "}}")
            }
            Self::PrimitiveOperation(po) => write!(f, "{:?}", po),
            Self::PrimitiveType(pt) => write!(f, "{:?}", pt),
            Self::PrimitiveValue(pv) => write!(f, "{:?}", pv),
            Self::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => {
                let gap = if f.alternate() { "\n" } else { "" };
                write!(f, "{:?} {}replacing{{", base, gap)?;
                for value in unlabeled_replacements {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{:?}", value)?;
                }
                for (target, value) in replacements {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{:?} with {:?} ", target, value)?;
                }
                write!(f, "{}}}", gap)
            }
            Self::TypeIs { exact, base, typee } => write!(
                f,
                "{:?} {}{:?}{}",
                base,
                if *exact { "type_is_exactly{" } else { ":" },
                typee,
                if *exact { "}" } else { "" },
            ),
            Self::Variable { selff, typee } => write!(f, "any{{{:?}}} at {:?}", typee, selff),
        }
    }
}
