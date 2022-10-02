use crate::shared::OrderedMap;

use super::ItemPtr;

pub type Substitutions = OrderedMap<ItemPtr, ItemPtr>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Equal {
    Yes(Substitutions),
    Unknown,
    No,
}

pub struct EqualityContext {
    other: ItemPtr,
}


