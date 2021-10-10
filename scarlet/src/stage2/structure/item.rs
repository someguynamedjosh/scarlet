use super::{BuiltinOperation, BuiltinValue, ItemId, OpaqueId};
use crate::shared::{OpaqueClass, OrderedMap};

pub type Definitions = OrderedMap<String, ItemId>;
pub type Substitutions = Vec<(Option<ItemId>, ItemId)>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Item {
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
    Match {
        base: ItemId,
        cases: Vec<(ItemId, ItemId)>,
    },
    Member {
        base: ItemId,
        name: String,
    },
    Opaque {
        class: OpaqueClass,
        id: OpaqueId,
        typee: ItemId,
    },
    Substituting {
        base: ItemId,
        substitutions: Substitutions,
    },
    TypeIs {
        base: ItemId,
        typee: ItemId,
    },
}
