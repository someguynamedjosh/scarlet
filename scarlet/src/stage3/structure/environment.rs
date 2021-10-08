use std::{fmt::Debug, path::PathBuf, str::FromStr};

use super::{AnnotatedValue, OpaqueValue, Value, ValueId};
use crate::shared::{OrderedSet, Pool};

#[derive(Clone, Debug)]
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

    pub fn write_debug_info(&self) {
        let out_path = PathBuf::from_str("ROOT.sir").unwrap();
        let contents = format!("{:#?}", self);
        std::fs::write(&out_path, contents).unwrap();
    }
}
