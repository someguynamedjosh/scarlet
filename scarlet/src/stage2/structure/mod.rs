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
    value: Option<Value>,
    typee: Option<ItemId>,
    defined_in: ScopeId,
    member_scopes: Vec<ScopeId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Scope {
    definition: ItemId,
    parent: Option<ScopeId>,
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
    definition: ItemId,
    original_type: ItemId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    definition: ItemId,
    original_type: ItemId,
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub items: Pool<Item>,
    pub scopes: Pool<Scope>,
    pub variables: Pool<Variable>,
    pub variant: Pool<Variant>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            items: Pool::new(),
            scopes: Pool::new(),
            variables: Pool::new(),
            variant: Pool::new(),
        }
    }
}
