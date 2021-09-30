use std::{
    fmt::{self, Debug},
    ops::{Index, IndexMut},
};

use crate::{
    shared::{Id, OrderedMap, Pool},
    util::indented,
};

mod value_debug;

pub type ItemReplacements = OrderedMap<ItemId, ItemId>;
pub type VariableReplacements = OrderedMap<VariableId, ItemId>;
pub type Definitions = OrderedMap<String, ItemId>;

pub type ItemId = Id<Item>;
pub type ScopeId = Id<Scope>;
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
    pub value: Option<Value>,
    pub typee: Option<ItemId>,
    pub defined_in: Option<ScopeId>,
    /// Ordered closest to farthest, I.E. the first ones should be searched
    /// first when looking for members.
    pub member_scopes: Vec<ScopeId>,
    pub cached_replacement: Option<ItemId>,
}

impl Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(value) = &self.value {
            let text = format!("{:#?}", value);
            let text = indented(&text);
            writeln!(f, "value: {}", text)?;
        }
        if let Some(typee) = &self.typee {
            writeln!(f, "typee: {:?}", typee)?;
        }
        writeln!(f, "in {:?}", self.defined_in)?;
        if self.member_scopes.len() > 0 {
            writeln!(f, "members from {:?}", self.member_scopes)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Scope {
    pub definition: ItemId,
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
        this_scope: ScopeId,
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
        name: String,
    },
    ReplacingItems {
        base: ItemId,
        replacements: ItemReplacements,
    },
    ReplacingVariables {
        base: ItemId,
        replacements: VariableReplacements,
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
    pub items: Pool<Item>,
    pub scopes: Pool<Scope>,
    pub variables: Pool<Variable>,
    pub variants: Pool<Variant>,
}

impl Environment {
    pub fn new() -> Self {
        let scopes = Pool::new();
        let items = Pool::new();
        Self {
            scopes,
            items,
            variables: Pool::new(),
            variants: Pool::new(),
        }
    }

    pub fn new_undefined_item(&mut self, defined_in: Option<ScopeId>) -> ItemId {
        if let Some(scope) = defined_in {
            assert!(self.scopes.contains(scope));
        }
        self.items.push(Item {
            defined_in,
            typee: None,
            value: None,
            member_scopes: Vec::new(),
            cached_replacement: None,
        })
    }

    pub fn define_item_value(&mut self, id: ItemId, value: Value) {
        assert!(self[id].value.is_none());
        self[id].value = Some(value)
    }

    pub fn insert_value(&mut self, defined_in: Option<ScopeId>, value: Value) -> ItemId {
        let id = self.new_undefined_item(defined_in);
        self.define_item_value(id, value);
        id
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
