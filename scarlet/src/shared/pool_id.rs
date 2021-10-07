use std::{
    fmt::{self, Debug},
    hash::Hash,
    marker::PhantomData,
};

use crate::shared::{reset_color, set_color_index};

pub struct Id<T, const C: char> {
    pub(super) pool_id: u64,
    pub index: usize,
    pub(super) _pd: PhantomData<T>,
}

impl<T, const C: char> Clone for Id<T, C> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            pool_id: self.pool_id,
            _pd: self._pd,
        }
    }
}

impl<T, const C: char> Copy for Id<T, C> {}

impl<T, const C: char> Debug for Id<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{} {}pool {} {}id {}{}>",
            C,
            set_color_index(self.pool_id as usize),
            self.pool_id,
            set_color_index(self.index),
            self.index,
            reset_color()
        )
    }
}

impl<T, const C: char> PartialEq for Id<T, C> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.pool_id == other.pool_id
    }
}

impl<T, const C: char> Eq for Id<T, C> {}

impl<T, const C: char> Hash for Id<T, C> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.index);
        state.write_u64(self.pool_id);
    }
}
