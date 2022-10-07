use std::iter::FromIterator;

use super::{query::QueryResult, ItemPtr};
use crate::{
    definitions::parameter::{Parameter, ParameterPtr},
    shared::OrderedSet,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Parameters {
    parameters: OrderedSet<ParameterPtr>,
    excludes_parameters_from: OrderedSet<ItemPtr>,
}

impl QueryResult for Parameters {
    fn is_final(&self) -> bool {
        !self.excludes_any_parameters()
    }
}

impl Parameters {
    pub fn new_empty() -> Self {
        Self {
            parameters: vec![].into_iter().collect(),
            excludes_parameters_from: vec![].into_iter().collect(),
        }
    }

    pub fn mark_excluding(&mut self, excluding_from: ItemPtr) {
        self.excludes_parameters_from.insert(excluding_from, ());
    }

    pub fn unmark_excluding(&mut self, no_longer_excluding_from: &ItemPtr) {
        self.excludes_parameters_from
            .remove(no_longer_excluding_from);
    }

    pub fn excludes_any_parameters(&self) -> bool {
        self.excludes_parameters_from.len() > 0
    }

    pub fn insert(&mut self, param: ParameterPtr) {
        self.parameters.insert(param, ());
    }

    pub fn contains(&self, param: &Parameter) -> bool {
        self.parameters.contains_key(param)
    }

    pub fn ordered(&self) -> Vec<&Parameter> {
        let mut ordered = Vec::from_iter(self.parameters.iter().map(|x| &*x.0));
        ordered.sort_by_key(|param| param.order());
        ordered
    }

    pub fn append(&mut self, mut other: Self) {
        for (param, _) in std::mem::take(&mut other.parameters) {
            self.insert(param);
        }
        for (excludes, _) in other.excludes_parameters_from.take() {
            self.excludes_parameters_from.insert(excludes, ());
        }
    }

    pub fn remove(&mut self, param: &Parameter) -> Option<ParameterPtr> {
        self.parameters.remove(param).map(|x| x.0)
    }
}
