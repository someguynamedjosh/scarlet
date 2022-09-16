use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet},
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

#[derive(Clone, PartialEq, Eq)]
pub struct PredicateSet {
    pub(super) predicates: PredicateIntersectionUnion,
}

impl Debug for PredicateSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PredicateSet")
            .field("predicates", &self.predicates)
            .finish_non_exhaustive()
    }
}

impl PredicateSet {
    pub fn new_empty() -> PredicateSet {
        Self {
            predicates: PredicateIntersectionUnion {
                base: vec![(
                    PredicateIntersection {
                        base: HashSet::new(),
                    },
                    (),
                )]
                .into_iter()
                .collect(),
            },
        }
    }

    pub fn push_and(&mut self, additional_predicate: ItemPtr) {
        self.predicates.push_and(additional_predicate)
    }

    pub fn push_or(&mut self, additional_predicate: ItemPtr) {
        self.predicates.push_or(additional_predicate)
    }

    pub fn union_with(&mut self, other: Self) {
        for (intersection, _) in other.predicates.base.into_iter() {
            self.predicates.push_or_intersection(intersection);
        }
    }

    pub fn intersection_with(&mut self, other: Self) {
        let self_intersections = self.predicates.base.take();
        let other_intersections = other.predicates.base.take();
        for (a, _) in &self_intersections {
            for (b, _) in &other_intersections {
                let a_b_intersection = a.base.union(&b.base);
                self.predicates.push_or_intersection(PredicateIntersection {
                    base: a_b_intersection.map(|x| x.ptr_clone()).collect(),
                });
            }
        }
    }
}
