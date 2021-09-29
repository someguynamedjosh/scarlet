use crate::{
    shared::{Item, ItemId},
    stage3,
};

mod environment_debug;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ItemDefinition {
    /// A list of all scopes from which info has been requested.
    pub info_requested_in: Vec<ItemId>,
    /// True if this item is a place where other items are defined.
    pub is_scope: bool,
    pub definition: Item,
    pub defined_in: Option<ItemId>,
    pub cached_type: Option<ItemId>,
    pub cached_reduction: Option<ItemId>,
}

impl ItemDefinition {
    pub fn from(other: stage3::structure::ItemDefinition) -> Self {
        Self {
            info_requested_in: other.info_requested_in,
            is_scope: other.is_scope,
            definition: other.definition,
            defined_in: other.defined_in,
            cached_type: None,
            cached_reduction: None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    pub items: Vec<ItemDefinition>,
}

impl Environment {
    pub fn from(other: stage3::structure::Environment) -> Self {
        let items = other.items.into_iter().map(ItemDefinition::from).collect();
        Self { items }
    }

    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &ItemDefinition)> {
        self.items
            .iter()
            .enumerate()
            .map(|(idx, val)| (ItemId(idx), val))
    }
}
