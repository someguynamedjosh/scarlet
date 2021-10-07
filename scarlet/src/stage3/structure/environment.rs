use std::fmt::{self, Debug};

use super::{AnnotatedValue, OpaqueValue, Value, ValueId};
use crate::shared::{OrderedSet, Pool};

#[derive(Clone)]
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

    /// Contextually format value
    pub fn cfv(&self, id: ValueId) -> String {
        self.values[id].value.contextual_fmt(self)
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (id, value) in &self.values {
            let value = value.contextual_fmt(self);
            write!(f, "\n{:?} is\n{}\n", id, value)?;
        }
        Ok(())
    }
}
