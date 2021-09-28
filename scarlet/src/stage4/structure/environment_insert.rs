use super::{Environment, TypedItem};
use crate::{
    shared::{Item, ItemId},
    stage4::ingest::var_list::VarList,
    util::MaybeResult,
};

impl Environment {
    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &TypedItem)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, val)| (ItemId(index), val))
    }

    pub fn _iter_mut(&mut self) -> impl Iterator<Item = (ItemId, &mut TypedItem)> {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(index, val)| (ItemId(index), val))
    }

    fn get_existing_item(&self, def: &Item) -> Option<ItemId> {
        for (index, item) in self.items.iter().enumerate() {
            if &item.definition == def {
                return Some(ItemId(index));
            }
        }
        None
    }

    pub fn insert(&mut self, def: Item, defined_in: Option<ItemId>) -> ItemId {
        if let Some(existing_id) = self.get_existing_item(&def) {
            return existing_id;
        }
        let id = ItemId(self.items.len());
        self.items.push(TypedItem {
            info_requested: None,
            is_scope: false,
            definition: def,
            defined_in,
            typee: None,
            reduction_blockers: VarList::new(),
        });
        id
    }

    pub fn insert_and_compute_type(
        &mut self,
        def: Item,
        defined_in: Option<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let id = self.insert(def, defined_in);
        self.compute_type(id, vec![])?;
        MaybeResult::Ok(id)
    }

    pub fn insert_with_type(
        &mut self,
        def: Item,
        typee: ItemId,
        defined_in: Option<ItemId>,
    ) -> ItemId {
        let id = ItemId(self.items.len());
        for (idx, item) in self.items.iter().enumerate() {
            if item.definition == def && item.typee == Some(typee) {
                return ItemId(idx);
            }
        }
        self.items.push(TypedItem {
            info_requested: None,
            is_scope: false,
            definition: def,
            defined_in,
            typee: Some(typee),
            reduction_blockers: VarList::new(),
        });
        id
    }

    pub fn set_type(&mut self, item: ItemId, typee: ItemId) {
        assert!(item.0 < self.items.len());
        self.items[item.0].typee = Some(typee)
    }
}
