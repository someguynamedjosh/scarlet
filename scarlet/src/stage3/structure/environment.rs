use super::{
    Namespace, NamespaceId, Replacements, ReplacementsId, Value, ValueId, Variable, VariableId,
    Variant, VariantId,
};

#[derive(Clone, Debug)]
pub struct Environment {
    pub namespaces: Pool<Namespace, 'N'>,
    pub replacements: Pool<Replacements, 'R'>,
    pub values: Pool<Value, 'L'>,
    pub variables: Pool<Variable, 'V'>,
    pub variants: Pool<Variant, 'T'>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            namespaces: Pool::new(),
            replacements: Pool::new(),
            values: Pool::new(),
            variables: Pool::new(),
            variants: Pool::new(),
        }
    }
}

impl Index<NamespaceId> for Environment {
    type Output = Namespace;

    fn index(&self, index: NamespaceId) -> &Self::Output {
        &self.namespaces[index]
    }
}

impl IndexMut<NamespaceId> for Environment {
    fn index_mut(&mut self, index: NamespaceId) -> &mut Self::Output {
        &mut self.namespaces[index]
    }
}

impl Index<ReplacementsId> for Environment {
    type Output = Replacements;

    fn index(&self, index: ReplacementsId) -> &Self::Output {
        &self.replacements[index]
    }
}

impl IndexMut<ReplacementsId> for Environment {
    fn index_mut(&mut self, index: ReplacementsId) -> &mut Self::Output {
        &mut self.replacements[index]
    }
}

impl Index<ValueId> for Environment {
    type Output = Value;

    fn index(&self, index: ValueId) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<ValueId> for Environment {
    fn index_mut(&mut self, index: ValueId) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl Index<VariableId> for Environment {
    type Output = Variable;

    fn index(&self, index: VariableId) -> &Self::Output {
        &self.variables[index]
    }
}

impl IndexMut<VariableId> for Environment {
    fn index_mut(&mut self, index: VariableId) -> &mut Self::Output {
        &mut self.variables[index]
    }
}

impl Index<VariantId> for Environment {
    type Output = Variant;

    fn index(&self, index: VariantId) -> &Self::Output {
        &self.variants[index]
    }
}

impl IndexMut<VariantId> for Environment {
    fn index_mut(&mut self, index: VariantId) -> &mut Self::Output {
        &mut self.variants[index]
    }
}
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use crate::shared::Pool;
