use crate::{
    shared::{Id, Pool},
    stage1::structure as s1,
};

#[derive(Clone, Debug)]
pub struct StructField<'x> {
    pub name: Option<String>,
    pub value: ItemId<'x>,
}

#[derive(Clone, Debug)]
pub struct Substitution<'x> {
    pub target: Option<ItemId<'x>>,
    pub value: ItemId<'x>,
}

#[derive(Clone, Debug)]
pub struct Condition<'x> {
    pub pattern: ItemId<'x>,
    pub value: ItemId<'x>,
}

#[derive(Clone, Copy, Debug)]
pub enum BuiltinOperation {
    Sum32U,
    Dif32U,
}

#[derive(Clone, Copy, Debug)]
pub enum BuiltinValue {
    GodPattern,
    _32UPattern,
    _32U(u32),
}

#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Item<'x> {
    pub original_definition: &'x s1::TokenTree<'x>,
    pub definition: Option<Definition<'x>>,
}

pub type VariableId<'x> = Id<Variable<'x>, 'V'>;
#[derive(Clone, Debug)]
pub struct Variable<'x> {
    pub pattern: ItemId<'x>,
}
