use std::ops::{Index, IndexMut};

use crate::shared::{Id, Pool};

mod definitions;
mod replacements;
mod value_debug;

pub use definitions::*;
pub use replacements::*;

pub type ItemId = Id<Item>;
pub type ScopeId = Id<Scope>;
pub type VariableId = Id<Variable>;
pub type VariantId = Id<Variant>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinOperation {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinValue {
    OriginType,
    I8Type,
    I8(i8),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Item {
    pub value: Option<Value>,
    pub typee: Option<ItemId>,
    pub defined_in: ScopeId,
    pub member_scopes: Vec<ScopeId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Scope {
    pub definition: Option<ItemId>,
    pub parent: Option<ScopeId>,
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
    Defining {
        base: ItemId,
        definitions: Definitions,
        child_scope: ScopeId,
    },
    FromItems {
        base: ItemId,
        items: Vec<ItemId>,
    },
    FromVariables {
        base: ItemId,
        variables: Vec<VariableId>,
    },
    Identifier {
        name: String,
    },
    Item {
        item: ItemId,
    },
    Member {
        base: ItemId,
        member: String,
    },
    Replacing {
        base: ItemId,
        replacements: Replacements,
    },
    Variant {
        variant: VariantId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub definition: ItemId,
    pub original_type: ItemId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub definition: ItemId,
    pub original_type: ItemId,
}

#[derive(Clone, Debug)]
pub struct Environment {
    root_scope: ScopeId,
    pub items: Pool<Item>,
    pub scopes: Pool<Scope>,
    pub variables: Pool<Variable>,
    pub variants: Pool<Variant>,
}

impl Environment {
    pub fn new() -> Self {
        let mut scopes = Pool::new();
        let root_scope = scopes.push(Scope {
            definition: None,
            parent: None,
        });
        Self {
            root_scope,
            items: Pool::new(),
            scopes,
            variables: Pool::new(),
            variants: Pool::new(),
        }
    }

    pub fn new_undefined_item(&mut self, defined_in: ScopeId) -> ItemId {
        assert!(self.scopes.contains(defined_in));
        self.items.push(Item {
            defined_in,
            typee: None,
            value: None,
            member_scopes: Vec::new(),
        })
    }

    pub fn define_item_value(&mut self, id: ItemId, value: Value) {
        self[id].value = Some(value)
    }

    pub fn get_root_scope(&self) -> ScopeId {
        self.root_scope
    }
}

impl Index<ItemId> for Environment {
    type Output = Item;
    fn index(&self, index: ItemId) -> &Self::Output {
        &self.items[index]
    }
}

impl IndexMut<ItemId> for Environment {
    fn index_mut(&mut self, index: ItemId) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl Index<ScopeId> for Environment {
    type Output = Scope;
    fn index(&self, index: ScopeId) -> &Self::Output {
        &self.scopes[index]
    }
}

impl IndexMut<ScopeId> for Environment {
    fn index_mut(&mut self, index: ScopeId) -> &mut Self::Output {
        &mut self.scopes[index]
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
