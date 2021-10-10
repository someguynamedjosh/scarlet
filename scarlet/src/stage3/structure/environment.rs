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
            if candidate.value.as_ref() == Some(&value) {
                return id;
            }
        }
        self.values.push(AnnotatedValue { value: Some(value) })
    }

    pub fn push_undefined_value(&mut self) -> ValueId {
        self.values.push(AnnotatedValue { value: None })
    }

    pub fn define_value(&mut self, id: ValueId, definition: Value) {
        assert!(self.values[id].value.is_none());
        self.values[id].value = Some(definition)
    }
}
