use super::{BuiltinOperation, Definitions, ItemId, PrimitiveType, PrimitiveValue, Replacements};

pub type ConditionalClause = (ItemId, ItemId);

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Defining {
        base: ItemId,
        definitions: Definitions,
    },
    FromType {
        base: ItemId,
        values: Vec<ItemId>,
    },
    GodType,
    VariantInstance {
        typee: ItemId,
        variant_id: ItemId,
        values: Vec<ItemId>,
    },
    Pick {
        initial_clause: ConditionalClause,
        elif_clauses: Vec<ConditionalClause>,
        else_clause: ItemId,
    },
    BuiltinOperation(BuiltinOperation),
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
    pub fn as_primitive_value(&self) -> Option<PrimitiveValue> {
        if let Self::PrimitiveValue(pv) = self {
            Some(*pv)
        } else {
            None
        }
    }
}
