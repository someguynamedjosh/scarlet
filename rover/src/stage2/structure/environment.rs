use super::Item;
use crate::shared::{ItemId, ResolvedItem};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    pub modules: Vec<ItemId>,
    pub(crate) items: Vec<Option<Item>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &Option<Item>)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, val)| (ItemId(index), val))
    }

    pub fn mark_as_module(&mut self, item: ItemId) {
        self.modules.push(item)
    }

    pub fn next_id(&mut self) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(None);
        id
    }

    pub fn insert(&mut self, definition: Item) -> ItemId {
        let id = self.next_id();
        self.define(id, definition);
        id
    }

    pub fn insert_variable(&mut self, typee: ItemId) -> ItemId {
        let selff = self.next_id();
        let definition = ResolvedItem::Variable { selff, typee }.into();
        self.define(selff, definition);
        selff
    }

    /// Turns the provided definitions into a Defining item with an extra item
    /// Self pointing to the inserted item.
    pub fn insert_self_referencing_define(
        &mut self,
        base: ItemId,
        mut definitions: Vec<(&str, ItemId)>,
    ) -> ItemId {
        let id = self.next_id();
        definitions.insert(0, ("Self", id));
        self.define(id, Item::defining(base, definitions));
        id
    }

    pub fn define(&mut self, item: ItemId, definition: Item) {
        assert!(item.0 < self.items.len());
        self.items[item.0] = Some(definition)
    }

    pub fn definition_of(&self, item: ItemId) -> &Option<Item> {
        assert!(item.0 < self.items.len());
        &self.items[item.0]
    }
}
