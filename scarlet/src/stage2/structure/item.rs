use super::{BuiltinOperation, BuiltinValue, VariableId, VariantId};
use crate::shared::{Id, OrderedMap};

pub type Definitions = OrderedMap<String, ItemId>;
pub type ItemId = Id<Item, 'I'>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Item {
    Any {
        typee: ItemId,
        id: VariableId,
    },
    BuiltinOperation(BuiltinOperation<ItemId>),
    BuiltinValue(BuiltinValue),
    Defining {
        base: ItemId,
        definitions: Definitions,
    },
    From {
        base: ItemId,
        value: ItemId,
    },
    Identifier(String),
    Member {
        base: ItemId,
        name: String,
    },
    Substituting {
        base: ItemId,
        target: ItemId,
        value: ItemId,
    },
    Variant {
        typee: ItemId,
        id: VariantId,
    },
}
