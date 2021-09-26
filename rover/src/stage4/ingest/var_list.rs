use crate::shared::ItemId;

pub struct VarList(Vec<ItemId>);

impl VarList {
    pub fn new() -> VarList {
        Self(Vec::new())
    }

    pub fn from(vec: Vec<ItemId>) -> VarList {
        Self(vec)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, item: ItemId) {
        if !self.0.contains(&item) {
            self.0.push(item)
        }
    }

    pub fn append(&mut self, items: &[ItemId]) {
        for item in items {
            self.push(*item);
        }
    }

    pub fn into_vec(self) -> Vec<ItemId> {
        self.0
    }

    pub fn as_slice(&self) -> &[ItemId] {
        &self.0[..]
    }

    pub fn pop_front(&mut self) -> Option<ItemId> {
        if self.len() == 0 {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}
