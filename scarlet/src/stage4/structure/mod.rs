use crate::{
    shared::{Item, ItemId},
    stage3,
};

mod environment_debug;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ItemDefinition {
    /// True when the programmer has requested a diagnostic showing information
    /// about this definition. Contains the scope from which the information was
    /// requested.
    pub info_requested: Option<ItemId>,
    /// True if this item is a place where other items are defined.
    pub is_scope: bool,
    pub definition: Item,
    pub defined_in: Option<ItemId>,
    pub cached_type: Option<ItemId>,
}

impl ItemDefinition {
    pub fn from(other: stage3::structure::ItemDefinition) -> Self {
        Self {
            info_requested: other.info_requested,
            is_scope: other.is_scope,
            definition: other.definition,
            defined_in: other.defined_in,
            cached_type: None,
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
