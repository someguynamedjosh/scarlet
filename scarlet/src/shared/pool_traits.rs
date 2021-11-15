use std::{
    fmt::{self, Debug},
    iter::{Enumerate, Map},
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use super::{Id, Pool};

impl<T: Clone, const C: char> Clone for Pool<T, C> {
    fn clone(&self) -> Self {
        Self {
            id: Self::next_pool_id(),
            items: self.items.clone(),
        }
    }
}

impl<T: Debug, const C: char> Debug for Pool<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pool<{}>[", C)?;
        for (id, element) in self {
            write!(f, "\n\nat {:?}\n", id)?;
            element.fmt(f)?;
        }
        write!(f, "\n\n]")
    }
}

impl<T, const C: char> Index<Id<C>> for Pool<T, C> {
    type Output = T;

    fn index(&self, index: Id<C>) -> &T {
        self.get(index)
    }
}

impl<T, const C: char> IndexMut<Id<C>> for Pool<T, C> {
    fn index_mut(&mut self, index: Id<C>) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<T, const C: char> IntoIterator for Pool<T, C> {
    type IntoIter = Map<Enumerate<IntoIter<T>>, Box<dyn Fn((usize, T)) -> (Id<C>, T)>>;
    type Item = (Id<C>, T);

    fn into_iter(self) -> Self::IntoIter {
        let pool_id = self.id;
        let mapper = move |(index, element)| (Id { index, pool_id }, element);
        self.items.into_iter().enumerate().map(Box::new(mapper))
    }
}

fn map_index_to_id<'a, T, const C: char>(
    pool_id: u64,
    (index, value): (usize, &'a T),
) -> (Id<C>, &'a T) {
    let id = unsafe { Pool::id_from_index(pool_id, index) };
    (id, value)
}

impl<'a, T: 'a, const C: char> IntoIterator for &'a Pool<T, C> {
    type IntoIter = Map<Enumerate<Iter<'a, T>>, Box<dyn Fn((usize, &'a T)) -> (Id<C>, &'a T)>>;
    type Item = (Id<C>, &'a T);

    fn into_iter(self) -> Self::IntoIter {
        let pool_id = self.id;
        let mapper = move |x| map_index_to_id(pool_id, x);
        self.items.iter().enumerate().map(Box::new(mapper))
    }
}

fn map_index_to_id_mut<'a, T, const C: char>(
    pool_id: u64,
    (index, value): (usize, &'a mut T),
) -> (Id<C>, &'a mut T) {
    let id = unsafe { Pool::id_from_index(pool_id, index) };
    (id, value)
}

impl<'a, T: 'a, const C: char> IntoIterator for &'a mut Pool<T, C> {
    type IntoIter =
        Map<Enumerate<IterMut<'a, T>>, Box<dyn Fn((usize, &'a mut T)) -> (Id<C>, &'a mut T)>>;
    type Item = (Id<C>, &'a mut T);

    fn into_iter(self) -> Self::IntoIter {
        let pool_id = self.id;
        let mapper = move |x| map_index_to_id_mut(pool_id, x);
        self.items.iter_mut().enumerate().map(Box::new(mapper))
    }
}
