use std::{
    fmt::{self, Debug},
    ops::{Index, IndexMut},
};

use crate::{
    shared::{Id, OrderedMap, OrderedSet, Pool},
    util::indented,
};

mod value_debug;

pub type Replacements = OrderedMap<ValueId, ValueId>;
pub type Definitions = OrderedMap<String, Item>;
pub type Variables = OrderedSet<VariableId>;

pub type NamespaceId = Id<Option<Namespace>>;
pub type ValueId = Id<Option<Value>>;
pub type VariableId = Id<Variable>;
pub type VariantId = Id<Variant>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinOperation {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinValue {
    OriginType,
    U8Type,
    U8(u8),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub namespace: NamespaceId,
    pub value: ValueId,
}

impl Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}, {:?}", self.namespace, self.value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Namespace {
    Defining {
        base: NamespaceId,
        definitions: Definitions,
        parent: NamespaceId,
    },
    Empty,
    Identifier(String),
    Member {
        base: NamespaceId,
        name: String,
    },
    Replacing {
        base: NamespaceId,
        replacements: Replacements,
    },
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Any {
        variable: VariableId,
    },
    BuiltinOperation {
        operation: BuiltinOperation,
    },
    BuiltinValue {
        value: BuiltinValue,
    },
    From {
        base: ValueId,
        values: Vec<ValueId>,
    },
    Identifier {
        name: String,
        scope: NamespaceId,
    },
    Member {
        base: NamespaceId,
        name: String,
    },
    Replacing {
        base: ValueId,
        replacements: Replacements,
    },
    Variant {
        variant: VariantId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub definition: ValueId,
    pub original_type: ValueId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub definition: ValueId,
    pub original_type: ValueId,
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub namespaces: Pool<Option<Namespace>>,
    pub values: Pool<Option<Value>>,
    pub variables: Pool<Variable>,
    pub variants: Pool<Variant>,
    pub info_requests: Vec<(Item, NamespaceId)>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            namespaces: Pool::new(),
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
