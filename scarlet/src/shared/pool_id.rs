use std::{
    fmt::{self, Debug},
    hash::Hash,
    marker::PhantomData,
};

use crate::shared::{reset_color, set_color_index};

pub struct Id<T> {
    pub(super) index: usize,
    pub(super) pool_id: u64,
    pub(super) _pd: PhantomData<T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            pool_id: self.pool_id,
            _pd: self._pd,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{} {}pool {} {}id {}{}>",
            std::any::type_name::<T>().split("::").last().unwrap(),
            set_color_index(self.pool_id as usize),
            self.pool_id,
            set_color_index(self.index),
            self.index,
            reset_color()
        )
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.pool_id == other.pool_id
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.index);
        state.write_u64(self.pool_id);
    }
}
