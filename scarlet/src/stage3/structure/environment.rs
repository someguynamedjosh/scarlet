use super::{Value, Variable, Variant};
use crate::shared::Pool;

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: Pool<Value, 'L'>,
    pub variables: Pool<Variable, 'V'>,
    pub variants: Pool<Variant, 'T'>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: Pool::new(),
            variables: Pool::new(),
            variants: Pool::new(),
        }
    }
}
