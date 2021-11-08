use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
};

use typed_arena::Arena;

use crate::{
    shared::{Id, OrderedMap, OrderedSet, Pool},
    stage1::structure::{self as s1},
    util::indented,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructField<'x> {
    pub name: Option<&'x str>,
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
    Difference32U,
    Product32U,
    Quotient32U,
    Modulo32U,
    Power32U,

    LessThan32U,
    LessThanOrEqual32U,
    GreaterThan32U,
    GreaterThanOrEqual32U,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VarType<'x> {
    God,
    _32U,
    Bool,
    Just(ItemId<'x>),
    And(ItemId<'x>, ItemId<'x>),
    Or(ItemId<'x>, ItemId<'x>),
}

impl<'x> VarType<'x> {
    pub fn map_item_ids(self, mut by: impl FnMut(ItemId<'x>) -> ItemId<'x>) -> Self {
        match self {
            Self::God | Self::_32U | Self::Bool => self,
            Self::Just(a) => Self::Just(by(a)),
            Self::And(a, b) => Self::And(by(a), by(b)),
            Self::Or(a, b) => Self::Or(by(a), by(b)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinValue {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VariableInfo<'x> {
    pub var_item: ItemId<'x>,
    pub var: VariableId<'x>,
    pub typee: VarType<'x>,
    pub eager: bool,
}

pub type Substitutions<'x> = OrderedMap<VariableId<'x>, ItemId<'x>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Member<'x> {
    Named(String),
    Index {
        index: ItemId<'x>,
        proof_lt_len: ItemId<'x>,
    },
}

pub type TokenStream<'x> = Vec<Token<'x>>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Token<'x> {
    Plain(&'x str),
    Item(ItemId<'x>),
    Stream {
        label: &'x str,
        contents: TokenStream<'x>,
    },
}

impl<'x> Token<'x> {
    pub fn unwrap_plain(&self) -> &'x str {
        if let Self::Plain(plain) = self {
            *plain
        } else {
            panic!("Expected a plain token, got {:?} instead", self)
        }
    }
}

impl<'x> Debug for Token<'x> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Plain(plain) => write!(f, "{}", plain),
            Token::Item(id) => write!(f, "{:?}", id),
            Token::Stream { label, contents } => {
                writeln!(f, "stream {} {{", label)?;
                for line in contents {
                    writeln!(f, "    {}", indented(&format!("{:?}", line)))?;
                }
                write!(f, "}}")
            }
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
    Member(ItemId<'x>, Member<'x>),
    Other(ItemId<'x>),
    Resolvable(Token<'x>),
    SetEager {
        base: ItemId<'x>,
        vals: Vec<ItemId<'x>>,
        eager: bool,
    },
    Struct(Vec<StructField<'x>>),
    Substitute(ItemId<'x>, Substitutions<'x>),
    Variable {
        var: VariableId<'x>,
        typee: VarType<'x>,
    },
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
    PartialItems(Vec<ItemId<'x>>),
    AllVars(OrderedSet<VariableId<'x>>),
}

pub type ItemId<'x> = Id<Item<'x>, 'I'>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item<'x> {
    pub original_definition: &'x Token<'x>,
    pub definition: Option<Definition<'x>>,
    pub scope: HashMap<&'x str, ItemId<'x>>,
    /// The variables this item's definition is dependent on.
    pub dependencies: Option<OrderedSet<VariableInfo<'x>>>,
    pub cached_reduction: Option<ItemId<'x>>,
    pub shown_from: Vec<ItemId<'x>>,
}

pub type VariableId<'x> = Id<Variable<'x>, 'V'>;
#[derive(Clone, Debug)]
pub struct Variable<'x> {
    pub pd: PhantomData<&'x ()>,
}
