use std::{
    fmt::{self, Debug},
    iter::{Enumerate, Map},
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use super::{Id, Pool};

impl<T: Clone> Clone for Pool<T> {
    fn clone(&self) -> Self {
        Self {
            id: Self::next_pool_id(),
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

impl<T> Index<Id<T>> for Pool<T> {
    type Output = T;

    fn index(&self, index: Id<T>) -> &T {
        self.get(index)
    }
}

impl<T> IndexMut<Id<T>> for Pool<T> {
    fn index_mut(&mut self, index: Id<T>) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<T> IntoIterator for Pool<T> {
    type IntoIter = Map<Enumerate<IntoIter<T>>, Box<dyn Fn((usize, T)) -> (Id<T>, T)>>;
    type Item = (Id<T>, T);

    fn into_iter(self) -> Self::IntoIter {
        let pool_id = self.id;
        let mapper = move |(index, element)| {
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

fn map_index_to_id<'a, T>(pool_id: u64, (index, value): (usize, &'a T)) -> (Id<T>, &'a T) {
    let id = unsafe { Pool::id_from_index(pool_id, index) };
    (id, value)
}

impl<'a, T: 'a> IntoIterator for &'a Pool<T> {
    type IntoIter = Map<Enumerate<Iter<'a, T>>, Box<dyn Fn((usize, &'a T)) -> (Id<T>, &'a T)>>;
    type Item = (Id<T>, &'a T);

    fn into_iter(self) -> Self::IntoIter {
        let pool_id = self.id;
        let mapper = move |x| map_index_to_id(pool_id, x);
        self.items.iter().enumerate().map(Box::new(mapper))
    }
}

fn map_index_to_id_mut<'a, T>(
    pool_id: u64,
    (index, value): (usize, &'a mut T),
) -> (Id<T>, &'a mut T) {
    let id = unsafe { Pool::id_from_index(pool_id, index) };
    (id, value)
}

impl<'a, T: 'a> IntoIterator for &'a mut Pool<T> {
    type IntoIter =
        Map<Enumerate<IterMut<'a, T>>, Box<dyn Fn((usize, &'a mut T)) -> (Id<T>, &'a mut T)>>;
    type Item = (Id<T>, &'a mut T);

    fn into_iter(self) -> Self::IntoIter {
        let pool_id = self.id;
        let mapper = move |x| map_index_to_id_mut(pool_id, x);
        self.items.iter_mut().enumerate().map(Box::new(mapper))
    }
}
