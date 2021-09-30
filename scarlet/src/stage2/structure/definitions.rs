use std::{
    fmt::{self, Debug},
    iter::FromIterator,
};

use super::ItemId;

pub type Definition = (String, ItemId);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Definitions(Vec<Definition>);

impl Definitions {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    fn insert_impl(&mut self, def: Definition, allow_replacement: bool) {
        if let Some(existing_idx) = self.0.iter().position(|i| i.0 == def.0) {
            if !allow_replacement {
                panic!("Tried value insert without replacement, but a definition for {:?} already exists.", def.0)
            }
            self.0[existing_idx].1 = def.1;
        } else {
            self.0.push((def.0, def.1))
        }
    }

    pub fn insert_or_replace(&mut self, def: Definition) {
        self.insert_impl(def, true)
    }

    #[track_caller]
    pub fn insert_no_replace(&mut self, def: Definition) {
        self.insert_impl(def, false)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        for (candidate, _) in self {
            if candidate == key {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &Definition> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn after_inserting(mut self, others: &Definitions) -> Self {
        for def in others {
            self.insert_or_replace((def.0.clone(), def.1));
        }
        self
    }
}

impl Debug for Definitions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "defining{{\n")?;
        for (name, value) in &self.0 {
            write!(f, "    {:?} is {:?}\n", name, value)?;
        }
        write!(f, "}}")
    }
}

impl Default for Definitions {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Definitions {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = (String, ItemId);

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Definitions {
    type IntoIter = std::slice::Iter<'a, (String, ItemId)>;
    type Item = &'a (String, ItemId);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromIterator<Definition> for Definitions {
    fn from_iter<T: IntoIterator<Item = Definition>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
