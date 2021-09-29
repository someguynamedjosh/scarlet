use std::{
    fmt::{self, Debug},
    hash::Hash,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

use crate::shared::{reset_color, set_color_index};

lazy_static! {
    static ref POOL_ID_COUNTER: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
}

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
            "<{} {}id {} {}pool {}{}>",
            std::any::type_name::<T>().split("::").last().unwrap(),
            set_color_index(self.index),
            self.index,
            set_color_index(self.pool_id as usize),
            self.pool_id,
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

pub struct Pool<T> {
    pub(super) id: u64,
    pub(super) items: Vec<T>,
}

impl<T> Pool<T> {
    pub(super) fn next_pool_id() -> u64 {
        let mut counter = POOL_ID_COUNTER.lock().unwrap();
        let result = *counter;
        *counter += 1;
        result
    }

    pub fn new() -> Self {
        Self {
            id: Self::next_pool_id(),
            items: Vec::new(),
        }
    }

    /// Returns true if the given ID was created by this pool (and therefore
    /// will not trigger a panic when used with get()).
    pub fn contains(&self, id: Id<T>) -> bool {
        self.id == id.pool_id
    }

    /// Abuse unsafe here to tell people that you really shouldn't use this
    /// function unless you know what you're doing.
    pub unsafe fn next_id(&self) -> Id<T> {
        Id {
            index: self.items.len(),
            pool_id: self.id,
            _pd: PhantomData,
        }
    }

    pub fn push(&mut self, item: T) -> Id<T> {
        let id = unsafe { self.next_id() };
        self.items.push(item);
        id
    }

    /// Returns the next ID after the given ID, or None if there is no item with
    /// the new ID.
    pub fn next(&self, after: Id<T>) -> Option<Id<T>> {
        let next_index = after.index + 1;
        if next_index < self.items.len() {
            Some(Id {
                index: next_index,
                pool_id: self.id,
                _pd: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn get(&self, id: Id<T>) -> &T {
        assert_eq!(id.pool_id, self.id);
        // We will never provide methods to remove data from a pool.
        debug_assert!(id.index < self.items.len());
        &self.items[id.index]
    }

    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        assert_eq!(id.pool_id, self.id);
        // We will never provide methods to remove data from a pool.
        debug_assert!(id.index < self.items.len());
        &mut self.items[id.index]
    }
}
