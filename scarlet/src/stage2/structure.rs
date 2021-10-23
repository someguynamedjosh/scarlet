use std::fmt::Debug;

use typed_arena::Arena;

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
    Matches,
    Sum32U,
    Dif32U,
    _32UPattern,
    BoolPattern,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinValue {
    GodPattern,
    _32U(u32),
    Bool(bool),
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

pub struct WrappedArena<T>(pub Arena<T>);

impl<T> Debug for WrappedArena<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "arena")
    }
}

#[derive(Debug)]
pub struct Environment<'x> {
    pub items: Pool<Item<'x>, 'I'>,
    pub vars: Pool<Variable<'x>, 'V'>,
    query_stack: Vec<ItemId<'x>>,
    pub(super) vomited_tokens: WrappedArena<String>,
}

impl<'x> Environment<'x> {
    pub fn new() -> Self {
        Self {
            items: Pool::new(),
            vars: Pool::new(),
            query_stack: Vec::new(),
            vomited_tokens: WrappedArena(Arena::new()),
        }
    }

    pub(super) fn with_fresh_query_stack<T>(&mut self, op: impl FnOnce(&mut Self) -> T) -> T {
        let old = std::mem::take(&mut self.query_stack);
        let result = op(self);
        debug_assert_eq!(self.query_stack.len(), 0);
        self.query_stack = old;
        result
    }

    pub(super) fn with_query_stack_frame<T>(
        &mut self,
        frame: ItemId<'x>,
        op: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.query_stack.push(frame);
        let result = op(self);
        debug_assert_eq!(self.query_stack.pop(), Some(frame));
        result
    }

    pub(super) fn query_stack_contains(&self, item: ItemId<'x>) -> bool {
        self.query_stack.contains(&item)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum After<'x> {
    Unknown,
    Items(Vec<ItemId<'x>>),
    Vars(OrderedSet<VariableId<'x>>),
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
    pub after: After<'x>,
    pub cached_reduction: Option<ItemId<'x>>,
    pub shown_from: Vec<ItemId<'x>>,
}

pub type VariableId<'x> = Id<Variable<'x>, 'V'>;
#[derive(Clone, Debug)]
pub struct Variable<'x> {
    pub pattern: ItemId<'x>,
}
