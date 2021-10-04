use super::{BuiltinOperation, BuiltinValue, VariableId, VariantId};
use crate::shared::OrderedMap;

pub type Definitions = OrderedMap<String, Item>;
pub type Substitutions = Vec<(Item, Item)>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Item {
    Any {
        typee: Box<Item>,
        id: VariableId,
    },
    BuiltinOperation(Box<BuiltinOperation<Item>>),
    BuiltinValue(BuiltinValue),
    Defining {
        base: Box<Item>,
        definitions: Definitions,
    },
    From {
        base: Box<Item>,
        values: Vec<Item>,
    },
    Identifier(String),
    Member {
        base: Box<Item>,
        name: String,
    },
    Substituting {
        base: Box<Item>,
        substitutions: Substitutions,
    },
    Variant {
        typee: Box<Item>,
        id: VariantId,
    },
}
