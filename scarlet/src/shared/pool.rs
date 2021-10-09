use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use serde::Serialize;

use super::Id;

lazy_static! {
    static ref POOL_ID_COUNTER: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
}

#[derive(Serialize)]
pub struct Pool<T, const C: char> {
    pub(super) id: u64,
    pub(super) items: Vec<T>,
}

impl<T, const C: char> Pool<T, C> {
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

    pub fn iter(&self) -> impl Iterator<Item = (Id<T, C>, &T)> {
        self.into_iter()
    }

    /// Returns true if the given ID was created by this pool (and therefore
    /// will not trigger a panic when used with get()).
    pub fn contains(&self, id: Id<T, C>) -> bool {
        self.id == id.pool_id
    }

    /// Abuse unsafe here to tell people that you really shouldn't use this
    /// function unless you know what you're doing.
    pub(super) unsafe fn next_id(&self) -> Id<T, C> {
        Id {
            index: self.items.len(),
            pool_id: self.id,
            _pd: PhantomData,
        }
    }

    pub(super) unsafe fn id_from_index(self_id: u64, index: usize) -> Id<T, C> {
        Id {
            index,
            pool_id: self_id,
            _pd: PhantomData,
        }
    }

    pub fn push(&mut self, item: T) -> Id<T, C> {
        let id = unsafe { self.next_id() };
        self.items.push(item);
        id
    }

    pub fn first(&self) -> Option<Id<T, C>> {
        if self.items.len() == 0 {
            None
        } else {
            Some(Id {
                index: 0,
                pool_id: self.id,
                _pd: PhantomData,
            })
        }
    }

    /// Returns the next ID after the given ID, or None if there is no item with
    /// the new ID.
    pub fn next(&self, after: Id<T, C>) -> Option<Id<T, C>> {
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

    pub fn get(&self, id: Id<T, C>) -> &T {
        assert_eq!(id.pool_id, self.id);
        // We will never provide methods to remove data from a pool.
        debug_assert!(id.index < self.items.len());
        &self.items[id.index]
    }

    pub fn get_mut(&mut self, id: Id<T, C>) -> &mut T {
        assert_eq!(id.pool_id, self.id);
        // We will never provide methods to remove data from a pool.
        debug_assert!(id.index < self.items.len());
        &mut self.items[id.index]
    }
}

impl<T: PartialEq + Eq, const C: char> Pool<T, C> {
    pub fn get_or_push(&mut self, item: T) -> Id<T, C> {
        for (id, candidate) in &*self {
            if candidate == &item {
                return id;
            }
        }
        self.push(item)
    }
}
