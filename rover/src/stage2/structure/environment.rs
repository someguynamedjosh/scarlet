use super::UnresolvedItem;
use crate::shared::{Item, ItemId};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    /// Items that should be displayed to the programmer.
    pub infos: Vec<ItemId>,
    pub modules: Vec<ItemId>,
    pub(crate) items: Vec<Option<UnresolvedItem>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            infos: vec![],
            modules: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &Option<UnresolvedItem>)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, val)| (ItemId(index), val))
    }

    pub fn mark_info(&mut self, item: ItemId) {
        self.infos.push(item)
    }

    pub fn mark_as_module(&mut self, item: ItemId) {
        self.modules.push(item)
    }

    pub fn next_id(&mut self) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(None);
        id
    }

    pub fn insert(&mut self, definition: UnresolvedItem) -> ItemId {
        let id = self.next_id();
        self.define(id, definition);
        id
    }

    pub fn insert_variable(&mut self, typee: ItemId) -> ItemId {
        let selff = self.next_id();
        let definition = Item::Variable { selff, typee }.into();
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
        self.define(id, UnresolvedItem::defining(base, definitions));
        id
    }

    pub fn define(&mut self, item: ItemId, definition: UnresolvedItem) {
        assert!(item.0 < self.items.len());
        self.items[item.0] = Some(definition)
    }

    pub fn definition_of(&self, item: ItemId) -> &Option<UnresolvedItem> {
        assert!(item.0 < self.items.len());
        &self.items[item.0]
    }
}
