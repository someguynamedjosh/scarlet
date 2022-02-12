use std::{
    fmt::{self, Debug},
    hash::Hash,
};

use serde::Serialize;

use crate::shared::{reset_color, set_color_index};

#[derive(Serialize)]
pub struct Id</* T, */ const C: char> {
    pub pool_id: u64,
    pub index: usize,
    // pub(super) _pd: PhantomData<*const T>,
}

impl<const C: char> Clone for Id<C> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            pool_id: self.pool_id,
            // _pd: self._pd,
        }
    }
}

impl<const C: char> Copy for Id<C> {}

impl<const C: char> Debug for Id<C> {
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

impl<const C: char> PartialEq for Id<C> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.pool_id == other.pool_id
    }
}

impl<const C: char> Eq for Id<C> {}

impl<const C: char> Hash for Id<C> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.index);
        state.write_u64(self.pool_id);
    }
}
