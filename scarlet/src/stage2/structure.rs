use crate::{
    shared::{Id, OrderedSet, Pool},
    stage1::structure as s1,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructField<'x> {
    pub name: Option<String>,
    pub value: ItemId<'x>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Substitution<'x> {
    pub target: Option<ItemId<'x>>,
    pub value: ItemId<'x>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Condition<'x> {
    pub pattern: ItemId<'x>,
    pub value: ItemId<'x>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinOperation {
    Sum32U,
    Dif32U,
    _32UPattern,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinValue {
    GodPattern,
    _32U(u32),
}

impl BuiltinValue {
    pub fn unwrap_32u(&self) -> u32 {
        match self {
            Self::_32U(value) => *value,
            _ => panic!("Expected 32U, got {:?} instead", self),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Definition<'x> {
    BuiltinOperation(BuiltinOperation, Vec<ItemId<'x>>),
    BuiltinValue(BuiltinValue),
    Match {
        base: ItemId<'x>,
        conditions: Vec<Condition<'x>>,
        else_value: ItemId<'x>,
    },
    Member(ItemId<'x>, String),
    Other(ItemId<'x>),
    Struct(Vec<StructField<'x>>),
    Substitute(ItemId<'x>, Vec<Substitution<'x>>),
    Variable(VariableId<'x>),
}

#[derive(Clone, Debug)]
pub struct Environment<'x> {
    pub items: Pool<Item<'x>, 'I'>,
    pub vars: Pool<Variable<'x>, 'V'>,
}

impl<'x> Environment<'x> {
    pub fn new() -> Self {
        Self {
            items: Pool::new(),
            vars: Pool::new(),
        }
    }
}

pub type ItemId<'x> = Id<Item<'x>, 'I'>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item<'x> {
    pub original_definition: &'x s1::TokenTree<'x>,
    pub definition: Option<Definition<'x>>,
    /// The variables this item's definition is dependent on.
    pub dependencies: Option<OrderedSet<VariableId<'x>>>,
    /// The variables that should remain dependencies when doing pattern
    /// matching.
    pub after: OrderedSet<VariableId<'x>>,
    pub cached_reduction: Option<ItemId<'x>>,
}

pub type VariableId<'x> = Id<Variable<'x>, 'V'>;
#[derive(Clone, Debug)]
pub struct Variable<'x> {
    pub pattern: ItemId<'x>,
}
