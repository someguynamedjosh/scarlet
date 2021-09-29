use std::{
    fmt::{self, Debug},
    hash::Hash,
    marker::PhantomData,
    slice::{Iter, IterMut},
};

use rand::RngCore;

pub struct Id<T> {
    index: usize,
    pool_id: u64,
    _pd: PhantomData<T>,
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
            "<{} id {} pool {}>",
            std::any::type_name::<T>(),
            self.index,
            self.pool_id
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
    id: u64,
    items: Vec<T>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self {
            id: rand::thread_rng().next_u64(),
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) -> Id<T> {
        self.items.push(item);
        Id {
            index: self.items.len() - 1,
            pool_id: self.id,
            _pd: PhantomData,
        }
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

impl<T: Clone> Clone for Pool<T> {
    fn clone(&self) -> Self {
        Self {
            id: rand::thread_rng().next_u64(),
            items: self.items.clone(),
        }
    }
}

impl<T: Debug> Debug for Pool<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pool<{}>[", std::any::type_name::<T>())?;
        for (id, element) in self {
            write!(f, "\n\nat {:?}\n", id)?;
            element.fmt(f)?;
        }
        write!(f, "\n\n]")
    }
}

use std::{
    iter::{Enumerate, Map},
    vec::IntoIter,
};

impl<T> IntoIterator for Pool<T> {
    type IntoIter = Map<Enumerate<IntoIter<T>>, Box<dyn Fn((usize, T)) -> (Id<T>, T)>>;
    type Item = (Id<T>, T);

    fn into_iter(self) -> Self::IntoIter {
        let pool_id = self.id;
        let mapper = |(index, element)| {
            (
                Id {
                    index,
                    pool_id,
                    _pd: PhantomData,
                },
                element,
            )
        };
        self.items.into_iter().enumerate().map(Box::new(mapper))
    }
}

impl<'a, T: 'a> IntoIterator for &'a Pool<T> {
    type IntoIter = Map<Enumerate<Iter<'a, T>>, Box<dyn Fn((usize, &'a T)) -> (Id<T>, &'a T)>>;
    type Item = (Id<T>, &'a T);

    fn into_iter(self) -> Self::IntoIter {
        let pool_id = self.id;
        let mapper = |(index, element)| {
            (
                Id {
                    index,
                    pool_id,
                    _pd: PhantomData,
                },
                element,
            )
        };
        self.items.iter().enumerate().map(Box::new(mapper))
    }
}

impl<'a, T: 'a> IntoIterator for &'a mut Pool<T> {
    type IntoIter =
        Map<Enumerate<IterMut<'a, T>>, Box<dyn Fn((usize, &'a mut T)) -> (Id<T>, &'a mut T)>>;
    type Item = (Id<T>, &'a mut T);

    fn into_iter(self) -> Self::IntoIter {
        let pool_id = self.id;
        let mapper = |(index, element)| {
            (
                Id {
                    index,
                    pool_id,
                    _pd: PhantomData,
                },
                element,
            )
        };
        self.items.iter_mut().enumerate().map(Box::new(mapper))
    }
}
