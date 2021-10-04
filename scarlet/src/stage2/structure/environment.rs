use crate::shared::{Id, Pool};

#[derive(Clone, Debug)]
pub struct Variable;

#[derive(Clone, Debug)]
pub struct Variant;

pub type VariableId = Id<Variable, 'V'>;
pub type VariantId = Id<Variant, 'T'>;

#[derive(Clone, Debug)]
pub struct Environment {
    pub variables: Pool<Variable, 'V'>,
    pub variants: Pool<Variant, 'T'>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: Pool::new(),
            variants: Pool::new(),
        }
    }

    pub fn new_variable(&mut self) -> VariableId {
        self.variables.push(Variable)
    }

    pub fn new_variant(&mut self) -> VariantId {
        self.variants.push(Variant)
    }
}
