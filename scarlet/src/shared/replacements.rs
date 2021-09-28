use std::fmt::{self, Debug};

use super::ItemId;

pub type Replacement = (ItemId, ItemId);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Replacements(Vec<Replacement>);

impl Replacements {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    fn insert_impl(&mut self, rep: Replacement, allow_replacement: bool) {
        if rep.0 == rep.1 {
            return;
        }
        for (_, replace_with) in &mut self.0 {
            if *replace_with == rep.0 {
                *replace_with = rep.1;
            }
        }
        if let Some(existing_idx) = self.0.iter().position(|i| i.0 == rep.0) {
            if !allow_replacement {
                panic!("Tried rep.1 insert without replacement, but a replacement for {:?} already exists.", rep.0)
            }
            self.0[existing_idx].1 = rep.1;
        } else {
            self.0.push(rep)
        }
    }

    pub fn insert_or_replace(&mut self, rep: Replacement) {
        self.insert_impl(rep, true)
    }

    #[track_caller]
    pub fn insert_no_replace(&mut self, rep: Replacement) {
        self.insert_impl(rep, false)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Replacement> {
        self.into_iter()
    }

    pub fn applied_to(&self, base: ItemId) -> ItemId {
        for rep in &self.0 {
            if rep.0 == base {
                return rep.1;
            }
        }
        base
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn after_inserting(mut self, others: &Replacements) -> Self {
        for rep in others {
            self.insert_or_replace(*rep);
        }
        self
    }
}

impl Debug for Replacements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "replacing{{\n")?;
        for rep in &self.0 {
            write!(f, "    {:?} with {:?}\n", rep.0, rep.1)?;
        }
        write!(f, "}}")
    }
}

impl Default for Replacements {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Replacements {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Replacement;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Replacements {
    type IntoIter = std::slice::Iter<'a, Replacement>;
    type Item = &'a Replacement;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
