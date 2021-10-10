use std::fmt::Debug;

use serde::Serialize;

use super::{AnnotatedValue, OpaqueValue, Value, ValueId};
use crate::shared::{OrderedSet, Pool};

#[derive(Clone, Debug, Serialize)]
pub struct Environment {
    pub values: Pool<AnnotatedValue, 'L'>,
    pub opaque_values: Pool<OpaqueValue, 'O'>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: Pool::new(),
            opaque_values: Pool::new(),
        }
    }

    pub fn get_or_push_value(&mut self, value: Value) -> ValueId {
        for (id, candidate) in &self.values {
            if candidate.value == value {
                return id;
            }
        }
        self.values.push(AnnotatedValue {
            cached_reduction: None,
            cached_type: None,
            defined_at: OrderedSet::new(),
            referenced_at: OrderedSet::new(),
            display_requested_from: OrderedSet::new(),
            value,
        })
    }
}
