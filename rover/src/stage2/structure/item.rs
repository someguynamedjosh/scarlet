use super::{Definitions, ItemId, PrimitiveOperation, PrimitiveType, PrimitiveValue, Replacements};

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
    Item(ItemId),
    Member {
        base: ItemId,
        name: String,
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
        unlabeled_replacements: Vec<ItemId>,
        replacements: Replacements,
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

impl Item {
    pub fn defining(base: ItemId, definitions: Vec<(&str, ItemId)>) -> Self {
        let definitions = definitions
            .into_iter()
            .map(|(name, val)| (name.to_owned(), val))
            .collect();
        Self::Defining { base, definitions }
    }
}
