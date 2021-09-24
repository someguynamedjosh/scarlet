use super::UnresolvedItem;
use crate::shared::{Item, ItemId};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ItemDefinition {
    /// True when the programmer has requested a diagnostic showing information
    /// about this definition.
    pub info_requested: bool,
    /// True if this item is a place where other items are defined.
    pub is_scope: bool,
    pub definition: Option<UnresolvedItem>,
    pub defined_in: Option<ItemId>,
}

impl ItemDefinition {
    pub fn new() -> Self {
        Self {
            info_requested: false,
            is_scope: false,
            definition: None,
            defined_in: None,
        }
    }

    pub fn with_is_scope(mut self) -> Self {
        self.is_scope = true;
        self
    }

    pub fn with_definition(mut self, definition: UnresolvedItem) -> Self {
        self.definition = Some(definition);
        self
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    pub items: Vec<ItemDefinition>,
}

impl Environment {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &ItemDefinition)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, val)| (ItemId(index), val))
    }

    pub fn mark_info(&mut self, item: ItemId) {
        assert!(item.0 < self.items.len());
        self.items[item.0].info_requested = true;
    }

    pub fn mark_as_scope(&mut self, item: ItemId) {
        assert!(item.0 < self.items.len());
        self.items[item.0].is_scope = true;
    }

    pub fn next_id(&mut self) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(ItemDefinition::new());
        id
    }

    pub fn insert(&mut self, definition: ItemDefinition) -> ItemId {
        let id = self.next_id();
        self.items[id.0] = definition;
        id
    }

    pub fn insert_unresolved_item(&mut self, item: UnresolvedItem) -> ItemId {
        self.insert(ItemDefinition::new().with_definition(item))
    }

    pub fn insert_item(&mut self, item: Item) -> ItemId {
        self.insert(ItemDefinition::new().with_definition(item.into()))
    }

    pub fn insert_scope(&mut self, item: Item) -> ItemId {
        self.insert(
            ItemDefinition::new()
                .with_definition(item.into())
                .with_is_scope(),
        )
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
        self.mark_as_scope(id);
        id
    }

    pub fn define(&mut self, item: ItemId, definition: UnresolvedItem) {
        assert!(item.0 < self.items.len());
        self.items[item.0].definition = Some(definition)
    }

    pub fn set_defined_in(&mut self, item: ItemId, defined_in: ItemId) {
        assert!(item.0 < self.items.len());
        assert!(defined_in.0 < self.items.len());
        self.items[item.0].defined_in = Some(defined_in);
    }

    pub fn definition_of(&self, item: ItemId) -> &ItemDefinition {
        assert!(item.0 < self.items.len());
        &self.items[item.0]
    }
}
