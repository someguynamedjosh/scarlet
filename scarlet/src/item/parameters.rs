use std::{collections::HashSet, iter::FromIterator, rc::Rc};

use maplit::hashset;

use super::query::QueryResult;
use crate::{definitions::parameter::Parameter, shared::OrderedSet};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Parameters {
    parameters: OrderedSet<Rc<Parameter>>,
}

impl QueryResult for Parameters {
    fn is_final(&self) -> bool {
        true
    }
}

impl Parameters {
    pub fn new_empty() -> Self {
        Self {
            parameters: vec![].into_iter().collect(),
        }
    }

    pub fn insert(&mut self, param: Rc<Parameter>) {
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
    }
}
