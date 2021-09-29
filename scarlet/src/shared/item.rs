use super::{BuiltinOperation, BuiltinValue, Definitions, ItemId, Replacements, VarList};

pub type ConditionalClause = (ItemId, ItemId);

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Any {
        selff: ItemId,
        typee: ItemId,
    },
    BuiltinOperation(BuiltinOperation),
    BuiltinValue(BuiltinValue),
    Defining {
        base: ItemId,
        definitions: Definitions,
    },
    FromType {
        base: ItemId,
        vals: VarList,
    },
    Pick {
        clauses: Vec<ConditionalClause>,
        default: ItemId,
    },
    Replacing {
        base: ItemId,
        replacements: Replacements,
    },
    TypeIs {
        base_type_only: bool,
        base: ItemId,
        typee: ItemId,
    },
    Variant {
        selff: ItemId,
        typee: ItemId,
    },
}

impl Item {
    pub fn as_builtin_value(&self) -> Option<BuiltinValue> {
        if let Self::BuiltinValue(pv) = self {
            Some(*pv)
        } else {
            None
        }
    }
}
