use super::{
    Item, Namespace, NamespaceId, Replacements, ReplacementsId, Value, ValueId, Variable,
    VariableId, Variant, VariantId,
};

#[derive(Clone, Debug)]
pub struct Environment {
    pub namespaces: Pool<Option<Namespace>>,
    pub replacements: Pool<Replacements>,
    pub values: Pool<Option<Value>>,
    pub variables: Pool<Variable>,
    pub variants: Pool<Variant>,
    pub info_requests: Vec<(Item, NamespaceId)>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            namespaces: Pool::new(),
            replacements: Pool::new(),
            values: Pool::new(),
            variables: Pool::new(),
            variants: Pool::new(),
            info_requests: Vec::new(),
        }
    }

    pub fn new_undefined_namespace(&mut self) -> NamespaceId {
        self.namespaces.push(None)
    }

    pub fn define_namespace(&mut self, id: NamespaceId, namespace: Namespace) {
        assert!(self[id].is_none());
        self[id] = Some(namespace)
    }

    pub fn insert_namespace(&mut self, namespace: Namespace) -> NamespaceId {
        self.namespaces.push(Some(namespace))
    }

    pub fn new_undefined_value(&mut self) -> ValueId {
        self.values.push(None)
    }

    pub fn define_value(&mut self, id: ValueId, value: Value) {
        assert!(self[id].is_none());
        self[id] = Some(value)
    }

    pub fn insert_value(&mut self, value: Value) -> ValueId {
        self.values.push(Some(value))
    }
}

impl Index<ValueId> for Environment {
    type Output = Option<Value>;

    fn index(&self, index: ValueId) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<ValueId> for Environment {
    fn index_mut(&mut self, index: ValueId) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl Index<NamespaceId> for Environment {
    type Output = Option<Namespace>;

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
