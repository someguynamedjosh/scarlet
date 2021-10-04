use std::collections::HashMap;

use super::{Value, ValueId, Variable, Variant};
use crate::shared::Pool;

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: Pool<Value, 'L'>,
    pub variables: Pool<Variable, 'V'>,
    pub variants: Pool<Variant, 'T'>,
    pub type_cache: HashMap<ValueId, ValueId>,
    pub reduce_cache: HashMap<ValueId, ValueId>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: Pool::new(),
            variables: Pool::new(),
            variants: Pool::new(),
            type_cache: HashMap::new(),
            reduce_cache: HashMap::new(),
        }
    }
}
