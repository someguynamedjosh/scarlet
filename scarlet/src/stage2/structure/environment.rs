use super::UnresolvedItem;
use crate::shared::ItemId;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ItemDefinition {
    /// True when the programmer has requested a diagnostic showing information
    /// about this definition. Contains the scope from which the information was
    /// requested.
    pub info_requested: Option<ItemId>,
    /// True if this item is a place where other items are defined.
    pub is_scope: bool,
    pub definition: Option<UnresolvedItem>,
    pub defined_in: Option<ItemId>,
}

impl ItemDefinition {
    pub fn new() -> Self {
        Self {
            info_requested: None,
            is_scope: false,
            definition: None,
            defined_in: None,
        }
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

    pub fn get(&self, id: ItemId) -> &ItemDefinition {
        &self.items[id.0]
    }

    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &ItemDefinition)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, val)| (ItemId(index), val))
    }

    pub fn mark_info(&mut self, item: ItemId, scope: Option<ItemId>) {
        assert!(item.0 < self.items.len());
        self.items[item.0].info_requested = scope;
    }

    pub fn next_id(&mut self) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(ItemDefinition::new());
        id
    }

    pub fn insert(&mut self, definition: ItemDefinition) -> ItemId {
        for (id, def) in self.iter() {
            if def == &definition {
                return id;
            }
        }
        let id = self.next_id();
        self.items[id.0] = definition;
        id
    }

    pub fn insert_unresolved_item(&mut self, item: UnresolvedItem) -> ItemId {
        self.insert(ItemDefinition::new().with_definition(item))
    }

    pub fn define(&mut self, item: ItemId, definition: UnresolvedItem) {
        assert!(item.0 < self.items.len());
        self.items[item.0].definition = Some(definition)
    }

    fn checked_set_defined_in(&mut self, item: ItemId, defined_in: ItemId) {
        if self.items[item.0].defined_in != Some(defined_in) {
            self.set_defined_in(item, defined_in);
        }
    }

    pub fn set_defined_in(&mut self, item: ItemId, defined_in: ItemId) {
        assert!(item.0 < self.items.len());
        assert!(defined_in.0 < self.items.len());
        self.items[item.0].defined_in = Some(defined_in);
        match &self.items[item.0].definition {
            Some(UnresolvedItem::Member { base, .. }) => {
                let base = *base;
                self.checked_set_defined_in(base, defined_in);
            }
            Some(UnresolvedItem::Item(base)) => {
                let base = *base;
                self.checked_set_defined_in(base, defined_in);
            }
            _ => (),
        }
    }
}
