use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

use crate::{
    shared::{Item, ItemId},
    stage3::structure::{self as stage3, ItemDefinition},
    stage4::ingest::var_list::VarList,
    util,
};

#[derive(Clone, PartialEq)]
pub struct TypedItem {
    /// True when the programmer has requested a diagnostic showing information
    /// about this definition. Contains the scope from which the information was
    /// requested.
    pub info_requested: Option<ItemId>,
    /// True if this item is a place where other items are defined.
    pub is_scope: bool,
    pub definition: Item,
    pub defined_in: Option<ItemId>,
    pub typee: Option<ItemId>,
}

#[derive(Clone, PartialEq)]
pub struct Environment {
    pub items: Vec<TypedItem>,
    pub item_reverse_lookup: HashMap<Item, ItemId>,
}

fn reverse_lookups(from: &stage3::Environment) -> HashMap<Item, ItemId> {
    from.items
        .iter()
        .enumerate()
        .map(|(index, item)| (item.definition.clone(), ItemId(index)))
        .collect()
}

fn items(items: Vec<ItemDefinition>) -> Vec<TypedItem> {
    items
        .into_iter()
        .map(|i| TypedItem {
            info_requested: i.info_requested,
            is_scope: i.is_scope,
            definition: i.definition,
            defined_in: i.defined_in,
            typee: None,
        })
        .collect()
}

impl Environment {
    pub fn _new_empty() -> Self {
        Self::new(stage3::Environment::new())
    }

    pub fn new(from: stage3::Environment) -> Self {
        Self {
            item_reverse_lookup: reverse_lookups(&from),
            items: items(from.items),
        }
    }

    pub fn deref_replacing_and_defining(&self, val: ItemId) -> ItemId {
        match &self.items[val.0].definition {
            Item::Defining { base, .. } | Item::Replacing { base, .. } => {
                self.deref_replacing_and_defining(*base)
            }
            _ => val,
        }
    }

    pub fn get(&self, id: ItemId) -> &TypedItem {
        assert!(id.0 < self.items.len());
        &self.items[id.0]
    }
}
