use std::fmt::{self, Debug, Formatter};

use crate::shared::{ItemId, ResolvedItem};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Resolved(ResolvedItem),
    Item(ItemId),
    Member { base: ItemId, name: String },
}

impl Item {
    pub fn defining(base: ItemId, definitions: Vec<(&str, ItemId)>) -> Self {
        Self::Resolved(ResolvedItem::defining(base, definitions))
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resolved(resolved) => resolved.fmt(f),
            Self::Item(id) => id.fmt(f),
            Self::Member { base, name } => write!(f, "{:?}::{}", base, name),
        }
    }
}

impl From<ResolvedItem> for Item {
    fn from(item: ResolvedItem) -> Self {
        Self::Resolved(item)
    }
}
