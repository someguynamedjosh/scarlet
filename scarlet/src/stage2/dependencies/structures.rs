use std::{fmt::Debug, hash::Hash};

use crate::{
    shared::OrderedSet,
    stage2::structure::{ItemId, VariableInfo},
};

#[derive(Debug)]
pub(super) struct QueryResult<'x, T> {
    pub(super) deps: OrderedSet<T>,
    pub(super) partial_over: OrderedSet<ItemId<'x>>,
}

pub(super) type DepQueryResult<'x> = QueryResult<'x, VariableInfo<'x>>;

impl<'x, T: PartialEq + Eq + Hash + Debug> QueryResult<'x, T> {
    pub fn new() -> Self {
        Self::empty(OrderedSet::new())
    }

    pub fn empty(partial_over: OrderedSet<ItemId<'x>>) -> Self {
        Self {
            deps: Default::default(),
            partial_over,
        }
    }

    pub fn full(vars: OrderedSet<T>) -> Self {
        Self {
            deps: vars,
            partial_over: OrderedSet::new(),
        }
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> QueryResult<'x, U> {
        QueryResult {
            deps: self.deps.into_iter().map(|(k, v)| (f(k), v)).collect(),
            partial_over: self.partial_over,
        }
    }

    pub fn append(&mut self, other: Self) {
        let sv = std::mem::take(&mut self.deps);
        self.deps = sv.union(other.deps);
        let spo = std::mem::take(&mut self.partial_over);
        self.partial_over = spo.union(other.partial_over);
    }

    pub fn remove_partial(&mut self, over: ItemId<'x>) {
        self.partial_over.remove(&over);
    }
}
