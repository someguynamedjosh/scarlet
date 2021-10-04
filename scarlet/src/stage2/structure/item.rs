use super::{BuiltinOperation, BuiltinValue, VariableId, VariantId};
use crate::shared::{Id, OrderedMap};

pub type Definitions = OrderedMap<String, ItemId>;
pub type ItemId = Id<Item, 'I'>;
pub type Substitutions = Vec<(ItemId, ItemId)>;

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
        values: Vec<ItemId>,
    },
    Identifier(String),
    Member {
        base: ItemId,
        name: String,
    },
    Substituting {
        base: ItemId,
        substitutions: Substitutions,
    },
    Variant {
        typee: ItemId,
        id: VariantId,
    },
}
