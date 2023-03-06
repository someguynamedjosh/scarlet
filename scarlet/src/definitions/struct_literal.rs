use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
};

use itertools::Itertools;

use super::compound_type::{DCompoundType, Type};
use crate::{diagnostic::Position, environment::ItemId, shared::TripleBool};

#[derive(Clone, Debug)]
pub struct DStructLiteral {
    fields: Vec<(String, ItemId)>,
    /// If true, a type is automatically generated based on the contents. If
    /// false, the type should be inferred.
    is_module: bool,
}

impl DStructLiteral {
    pub fn new_module(fields: Vec<(String, ItemId)>) -> Self {
        Self {
            fields,
            is_module: true,
        }
    }

    pub fn new_struct(fields: Vec<(String, ItemId)>) -> Self {
        Self {
            fields,
            is_module: false,
        }
    }

    pub fn is_module(&self) -> bool {
        self.is_module
    }

    pub fn fields(&self) -> &[(String, ItemId)] {
        &self.fields
    }

    pub fn get_field(&self, name: &str) -> Option<ItemId> {
        for (candidate_name, candidate_value) in &self.fields {
            if candidate_name == name {
                return Some(*candidate_value);
            }
        }
        None
    }
}
