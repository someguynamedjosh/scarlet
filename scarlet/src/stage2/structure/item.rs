use super::{BuiltinOperation, BuiltinValue};
use crate::shared::OrderedMap;

pub type Definitions = OrderedMap<String, Item>;
pub type Replacements = Vec<(Item, Item)>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Item {
    Any {
        typee: Box<Item>,
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
    Replacing {
        base: Box<Item>,
        replacements: Replacements,
    },
    Variant {
        typee: Box<Item>,
    },
}
