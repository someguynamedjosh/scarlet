use std::{
    collections::hash_map::DefaultHasher,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use super::{parameters::Parameters, ItemPtr, ItemQueryResultCaches, LazyItemPtr};
use crate::{diagnostic::Diagnostic, environment::OnlyConstructedByEnvironment};

pub trait QueryResult: Clone + Hash + Eq {
    /// Returns true when the query result will not change on future calls.
    fn is_final(&self) -> bool;
}

pub trait Query {
    type Result: QueryResult;
    type Target: Hash;

    fn result_when_cycle_encountered(target: &Self::Target) -> Self::Result;
}

pub struct QueryResultCache<Q: Query + ?Sized> {
    pub data: Option<Q::Result>,
}

impl<Q: Query + ?Sized> Clone for QueryResultCache<Q>
where
    Q::Result: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<Q: Query + ?Sized> Debug for QueryResultCache<Q>
where
    Q::Result: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("QueryResultCache")
            .field("data", &self.data)
            .finish()
    }
}

pub struct QueryContext<Q: Query + ?Sized> {
    pub cycle_detection_stack: Vec<u64>,
    phantom: PhantomData<Q>,
}

impl QueryContext<RootQuery> {
    pub(crate) fn root(_: OnlyConstructedByEnvironment) -> QueryContext<RootQuery> {
        QueryContext::new()
    }
}

impl<Q: Query + ?Sized> QueryContext<Q> {
    fn new() -> Self {
        Self {
            cycle_detection_stack: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn get_query_result<
        D: Deref<Target = QueryResultCache<Q>>,
        Dm: DerefMut<Target = QueryResultCache<Q>>,
    >(
        &mut self,
        key: &Q::Target,
        recompute_result: impl FnOnce(&mut Self) -> Q::Result,
        cache: impl FnOnce(&Q::Target) -> D,
        cache_mut: impl FnOnce(&Q::Target) -> Dm,
    ) -> Q::Result {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let key_hash = hasher.finish();
        if self.cycle_detection_stack.contains(&key_hash) {
            let result = Q::result_when_cycle_encountered(key);
            assert!(
                !result.is_final(),
                "Results returned when cycles are encountered should be temporary."
            );
            result
        } else {
            let cache = cache(key);
            if let Some(result) = cache.data.clone() {
                result
            } else {
                drop(cache);
                self.cycle_detection_stack.push(key_hash);
                let result = recompute_result(self);
                assert_eq!(self.cycle_detection_stack.pop(), Some(key_hash));
                let mut cache_mut = cache_mut(key);
                cache_mut.data = Some(result.clone());
                drop(cache_mut);
                result
            }
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

impl<T: Clone + Hash + Eq, E: Clone + Hash + Eq> QueryResult for Result<T, E> {
    fn is_final(&self) -> bool {
        self.is_ok()
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct InfallibleQueryResult<T>(pub T);

impl<T: Clone + Hash + Eq> InfallibleQueryResult<T> {
    fn into(self) -> T {
        self.0
    }
}

impl<T: Clone + Hash + Eq> QueryResult for InfallibleQueryResult<T> {
    fn is_final(&self) -> bool {
        true
    }
}

impl<T: Clone + Hash + Eq> From<T> for InfallibleQueryResult<T> {
    fn from(val: T) -> Self {
        Self(val)
    }
}

impl QueryResult for ! {
    fn is_final(&self) -> bool {
        *self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaybeTemporary<T> {
    value: T,
    temporary: bool,
}

impl<T> MaybeTemporary<T> {
    pub fn new_final(value: T) -> Self {
        Self {
            value,
            temporary: false,
        }
    }

    pub fn new_temporary(value: T) -> Self {
        Self {
            value,
            temporary: true,
        }
    }

    pub fn is_final(&self) -> bool {
        !self.temporary
    }

    pub fn is_temporary(&self) -> bool {
        self.temporary
    }

    pub fn into_final(self) -> Option<T> {
        if self.is_final() {
            Some(self.value)
        } else {
            None
        }
    }

    pub fn into_temporary(self) -> Option<T> {
        if self.is_temporary() {
            Some(self.value)
        } else {
            None
        }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> MaybeTemporary<U> {
        MaybeTemporary {
            value: f(self.value),
            temporary: self.temporary,
        }
    }
}

impl<A, B> MaybeTemporary<(A, B)> {
    pub fn and2(a: MaybeTemporary<A>, b: MaybeTemporary<B>) -> Self {
        Self {
            value: (a.value, b.value),
            temporary: a.temporary || b.temporary,
        }
    }
}

impl<A, B, C> MaybeTemporary<(A, B, C)> {
    pub fn and3(a: MaybeTemporary<A>, b: MaybeTemporary<B>, c: MaybeTemporary<C>) -> Self {
        Self {
            value: (a.value, b.value, c.value),
            temporary: a.temporary || b.temporary || c.temporary,
        }
    }
}

impl<A, B, C, D> MaybeTemporary<(A, B, C, D)> {
    pub fn and4(
        a: MaybeTemporary<A>,
        b: MaybeTemporary<B>,
        c: MaybeTemporary<C>,
        d: MaybeTemporary<D>,
    ) -> Self {
        Self {
            value: (a.value, b.value, c.value, d.value),
            temporary: a.temporary || b.temporary || c.temporary || d.temporary,
        }
    }
}

impl<T: Clone + PartialEq + Eq + Hash> QueryResult for MaybeTemporary<T> {
    fn is_final(&self) -> bool {
        self.is_final()
    }
}

pub struct ChildrenQuery;

impl Query for ChildrenQuery {
    type Result = InfallibleQueryResult<Vec<ItemPtr>>;
    type Target = ItemPtr;

    fn result_when_cycle_encountered(_target: &Self::Target) -> Self::Result {
        panic!("Child query should not cause cycles!")
    }
}

pub struct ParametersQuery;

impl Query for ParametersQuery {
    type Result = Parameters;
    type Target = ItemPtr;

    fn result_when_cycle_encountered(target: &Self::Target) -> Self::Result {
        let mut p = Parameters::new_empty();
        p.mark_excluding(target.ptr_clone());
        p
    }
}

/// This only exists to describe what queries can be dispatched by Environment.
pub struct RootQuery;

impl Query for RootQuery {
    type Result = !;
    type Target = !;

    fn result_when_cycle_encountered(_target: &Self::Target) -> Self::Result {
        panic!("Root query should never be dispatched.")
    }
}

pub struct TypeQuery;

impl Query for TypeQuery {
    type Result = Option<LazyItemPtr>;
    type Target = ItemPtr;

    fn result_when_cycle_encountered(_target: &Self::Target) -> Self::Result {
        None
    }
}

/// Checks that all children of an item have expected types. It is okay to call
/// query_type() from a type check query but it is not okay to call
/// query_type_check() from a type query.
pub struct TypeCheckQuery;

impl Query for TypeCheckQuery {
    type Result = InfallibleQueryResult<Vec<Diagnostic>>;
    type Target = ItemPtr;

    fn result_when_cycle_encountered(_target: &Self::Target) -> Self::Result {
        vec![].into()
    }
}

pub fn no_type_check_errors() -> <TypeCheckQuery as Query>::Result {
    vec![].into()
}

pub struct ResolveQuery;

impl Query for ResolveQuery {
    type Result = Result<ItemPtr, Diagnostic>;
    type Target = ItemPtr;

    fn result_when_cycle_encountered(target: &Self::Target) -> Self::Result {
        Ok(target.ptr_clone())
    }
}

pub trait AllowsChildQuery<ChildQuery: Query> {
    fn with_child_context<T>(
        &mut self,
        operation: impl FnOnce(&mut QueryContext<ChildQuery>) -> T,
    ) -> T;
}

impl<Q: Query> AllowsChildQuery<Q> for QueryContext<Q> {
    fn with_child_context<T>(&mut self, operation: impl FnOnce(&mut QueryContext<Q>) -> T) -> T {
        operation(self)
    }
}

macro_rules! allow_child_query {
    ($Parent:ty => $Child:ty) => {
        impl AllowsChildQuery<$Child> for QueryContext<$Parent> {
            fn with_child_context<T>(
                &mut self,
                operation: impl FnOnce(&mut QueryContext<$Child>) -> T,
            ) -> T {
                let mut ctx = QueryContext::new();
                operation(&mut ctx)
            }
        }
    };
}

allow_child_query!(RootQuery => ParametersQuery);
allow_child_query!(RootQuery => ResolveQuery);
allow_child_query!(RootQuery => TypeCheckQuery);
allow_child_query!(RootQuery => TypeQuery);
allow_child_query!(ParametersQuery => TypeQuery);
allow_child_query!(TypeCheckQuery => TypeQuery);
