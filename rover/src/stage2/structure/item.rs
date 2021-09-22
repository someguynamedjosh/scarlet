use std::fmt::{self, Debug, Formatter};

use crate::shared::{Item, ItemId};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum UnresolvedItem {
    Just(Item),
    Item(ItemId),
    Member { base: ItemId, name: String },
}

impl UnresolvedItem {
    pub fn defining(base: ItemId, definitions: Vec<(&str, ItemId)>) -> Self {
        Self::Just(Item::defining(base, definitions))
    }
}

impl Debug for UnresolvedItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Just(resolved) => resolved.fmt(f),
            Self::Item(id) => id.fmt(f),
            Self::Member { base, name } => write!(f, "{:?}::{}", base, name),
        }
    }
}

impl From<Item> for UnresolvedItem {
    fn from(item: Item) -> Self {
        Self::Just(item)
    }
}
