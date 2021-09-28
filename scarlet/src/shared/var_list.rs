use std::iter::FromIterator;

use super::ItemId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VarList(Vec<ItemId>);

impl VarList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, var: ItemId) {
        if !self.0.contains(&var) {
            self.0.push(var)
        }
    }

    pub fn append(&mut self, other: &VarList) {
        for id in &other.0 {
            self.push(*id)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &ItemId> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, id: ItemId) -> bool {
        self.0.contains(&id)
    }
}

impl Default for VarList {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for VarList {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = ItemId;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a VarList {
    type IntoIter = std::slice::Iter<'a, ItemId>;
    type Item = &'a ItemId;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromIterator<ItemId> for VarList {
    fn from_iter<T: IntoIterator<Item = ItemId>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
