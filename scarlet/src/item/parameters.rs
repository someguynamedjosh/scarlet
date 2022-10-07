use std::{iter::FromIterator, collections::HashMap};

use super::{query::QueryResult, ItemPtr};
use crate::{
    definitions::parameter::{Parameter, ParameterPtr},
    shared::OrderedSet,
    util::PtrExtension,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Parameters {
    parameters: OrderedSet<(ItemPtr, ParameterPtr)>,
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

    pub fn insert(&mut self, reduced_type: ItemPtr, param: ParameterPtr) {
        self.parameters.insert((reduced_type, param), ());
    }

    pub fn contains(&self, param: &Parameter) -> bool {
        self.parameters.iter().any(|((_, p), _)| &**p == param)
    }

    pub fn ordered(&self) -> Vec<&Parameter> {
        let mut ordered = Vec::from_iter(self.parameters.iter().map(|x| &*x.0 .1));
        ordered.sort_by_key(|param| param.order());
        ordered
    }

    pub fn append(&mut self, mut other: Self) {
        for (param, _) in std::mem::take(&mut other.parameters) {
            self.insert(param.0, param.1);
        }
        for (excludes, _) in other.excludes_parameters_from.take() {
            self.excludes_parameters_from.insert(excludes, ());
        }
    }

    pub fn remove(&mut self, param: &Parameter) -> Option<(ItemPtr, ParameterPtr)> {
        let key = self.parameters.iter().find(|x| &*x.0 .1 == param)?;
        self.parameters.remove(&key.0.clone()).map(|x| x.0)
    }

    pub fn reduce_type(&mut self, args: &HashMap<ParameterPtr, ItemPtr>) {
        for (param, _) in self.parameters.iter_mut() {
            param.0 = param.0.reduce(args);
        }
    }
}
