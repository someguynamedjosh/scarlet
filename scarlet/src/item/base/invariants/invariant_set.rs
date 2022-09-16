use std::{
    cell::RefCell,
    collections::{BTreeSet, HashSet},
    fmt::{self, Debug, Formatter},
    hash::Hash,
    rc::Rc,
};

use itertools::Itertools;
use maplit::hashset;

use crate::{
    item::{dependencies::Dependencies, ItemPtr},
    shared::OrderedSet,
    util::rcrc,
};

type All<T> = HashSet<T>;
type Any<T> = HashSet<T>;

/// Represents a compound predicate which is true only when all its component
/// predicates are true.
#[derive(Clone, Debug, PartialEq, Eq)]
struct PredicateIntersection {
    base: HashSet<ItemPtr>,
}

impl Hash for PredicateIntersection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut addresses = self
            .base
            .iter()
            .map(|item| item.as_ptr().to_bits())
            .collect_vec();
        addresses.sort();
        addresses.hash(state);
    }
}

impl PredicateIntersection {
    pub fn push_and(&mut self, additional_predicate: ItemPtr) {
        self.base.insert(additional_predicate);
    }
}

/// Represents a compound predicate which is true only when at least one of its
/// component intersections is true.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct PredicateIntersectionUnion {
    base: OrderedSet<PredicateIntersection>,
}

impl PredicateIntersectionUnion {
    pub fn push_and(&mut self, additional_predicate: ItemPtr) {
        for (intersection, ()) in std::mem::take(&mut self.base).into_iter() {
            intersection.push_and(additional_predicate);
            self.base.insert_or_replace(intersection, ());
        }
    }

    pub fn push_or(&mut self, additional_predicate: ItemPtr) {
        self.base.insert_or_replace(
            PredicateIntersection {
                base: hashset![additional_predicate],
            },
            (),
        );
    }

    pub fn push_or_intersection(&mut self, intersection: PredicateIntersection) {
        self.base.insert_or_replace(intersection, ());
    }
}

#[derive(Clone)]
pub struct InvariantSet {
    pub(super) target: ItemPtr,
    pub(super) predicates: PredicateIntersectionUnion,
}

impl PartialEq for InvariantSet {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
    }
}

impl Debug for InvariantSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("InvariantSet")
            .field("context", &self.target.debug_label())
            .field("statements", &self.predicates)
            .finish_non_exhaustive()
    }
}

pub type InvariantSetPtr = Rc<RefCell<InvariantSet>>;

impl InvariantSet {
    pub fn new_empty(target: ItemPtr) -> InvariantSet {
        Self {
            target,
            predicates: Default::default(),
        }
    }

    pub fn target(&self) -> &ItemPtr {
        &self.target
    }

    pub fn push_and(&mut self, additional_predicate: ItemPtr) {
        self.predicates.push_and(additional_predicate)
    }

    pub fn push_or(&mut self, additional_predicate: ItemPtr) {
        self.predicates.push_or(additional_predicate)
    }

    pub fn union_with(&mut self, other: Self) {
        assert!(self.target.is_same_instance_as(&other.target));
        for (intersection, _) in other.predicates.base.into_iter() {
            self.predicates.push_or_intersection(intersection);
        }
    }
}
