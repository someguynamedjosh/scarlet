use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use super::ItemPtr;

pub trait QueryResult: Clone + Hash + Eq {
    /// Returns true when the query result will not change on future calls.
    fn is_final(&self) -> bool;
}

pub trait Query {
    type Result: QueryResult;
    type Target: Hash;

    fn result_when_cycle_encountered() -> Self::Result;
}

pub struct QueryResultCache<Q: Query + ?Sized> {
    data: Option<Q::Result>,
}

pub struct QueryContext<Q: Query + ?Sized> {
    cycle_detection_stack: Vec<u64>,
    phantom: PhantomData<Q>,
}

impl<Q: Query + ?Sized> QueryContext<Q> {
    pub fn new() -> Self {
        Self {
            cycle_detection_stack: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn get_query_result(
        &mut self,
        key: &impl Hash,
        recompute_query_result: impl FnOnce(&mut Self) -> Q::Result,
        cache: &mut QueryResultCache<Q>,
    ) -> Q::Result {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let key = hasher.finish();
        if self.cycle_detection_stack.contains(&key) {
            let result = Q::result_when_cycle_encountered();
            assert!(
                !result.is_final(),
                "Results returned when cycles are encountered should be temporary."
            );
            result
        } else {
            self.cycle_detection_stack.push(key);
            let result = cache.get_query_result(|| recompute_query_result(self));
            assert_eq!(self.cycle_detection_stack.pop(), Some(key));
            result
        }
    }
}

impl<Q: Query + ?Sized> QueryResultCache<Q> {
    pub fn new() -> Self {
        Self { data: None }
    }

    fn get_query_result(
        &mut self,
        recompute_query_result: impl FnOnce() -> Q::Result,
    ) -> Q::Result {
        if let Some(data) = &self.data {
            data.clone()
        } else {
            let data = recompute_query_result();
            if data.is_final() {
                self.data = Some(data.clone());
            }
            data
        }
    }
}

impl<T: Clone + Hash + Eq> QueryResult for Option<T> {
    fn is_final(&self) -> bool {
        self.is_some()
    }
}

pub struct TypeQuery;

impl Query for TypeQuery {
    type Result = Option<ItemPtr>;
    type Target = ItemPtr;

    fn result_when_cycle_encountered() -> Self::Result {
        None
    }
}
