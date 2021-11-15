use std::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
};

use typed_arena::Arena;

use super::construct::Construct;
use crate::{
    shared::{Id, OrderedMap, OrderedSet, Pool},
    util::indented,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Condition<'x> {
    pub pattern: ConstructId<'x>,
    pub value: ConstructId<'x>,
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
    Just(ConstructId<'x>),
    And(ConstructId<'x>, ConstructId<'x>),
    Or(ConstructId<'x>, ConstructId<'x>),
    Array {
        length: ConstructId<'x>,
        element_type: ConstructId<'x>,
    },
}

impl<'x> VarType<'x> {
    pub fn map_item_ids(self, mut by: impl FnMut(ConstructId<'x>) -> ConstructId<'x>) -> Self {
        match self {
            Self::God | Self::_32U | Self::Bool => self,
            Self::Just(a) => Self::Just(by(a)),
            Self::And(a, b) => Self::And(by(a), by(b)),
            Self::Or(a, b) => Self::Or(by(a), by(b)),
            Self::Array {
                length,
                element_type,
            } => Self::Array {
                length: by(length),
                element_type: by(element_type),
            },
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
    pub var_item: ConstructId<'x>,
    pub var: VariableId<'x>,
    pub typee: VarType<'x>,
    pub eager: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Member<'x> {
    Named(String),
    Index {
        index: ConstructId<'x>,
        proof_lt_len: ConstructId<'x>,
    },
}

pub type TokenStream<'x> = Vec<Token<'x>>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Token<'x> {
    Plain(&'x str),
    Item(ConstructId<'x>),
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

    pub fn unwrap_stream(&self) -> &TokenStream<'x> {
        if let Self::Stream { contents, .. } = self {
            contents
        } else {
            panic!("Expected a stream token, got {:?} instead", self)
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

pub struct WrappedArena<T>(pub Arena<T>);

impl<T> Debug for WrappedArena<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "arena")
    }
}

#[derive(Debug)]
pub struct Environment<'x> {
    pub items: Pool<AnnotatedConstruct<'x>, 'I'>,
    pub vars: Pool<Variable<'x>, 'V'>,
    query_stack: Vec<ConstructId<'x>>,
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

    pub fn push_item(&mut self, item: AnnotatedConstruct<'x>) -> ConstructId<'x> {
        let id = self.items.push(item);
        self.check(id);
        id
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
        frame: ConstructId<'x>,
        op: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.query_stack.push(frame);
        let result = op(self);
        debug_assert_eq!(self.query_stack.pop(), Some(frame));
        result
    }

    pub(super) fn query_stack_contains(&self, item: ConstructId<'x>) -> bool {
        self.query_stack.contains(&item)
    }
}

pub type ConstructId<'x> = Id<AnnotatedConstruct<'x>, 'I'>;
#[derive(Debug)]
pub struct AnnotatedConstruct<'x> {
    pub definition: Option<Box<dyn Construct<'x> + 'x>>,
    pub parent_scope: Option<ConstructId<'x>>,
    /// The variables this item's definition is dependent on.
    pub dependencies: Option<OrderedSet<VariableInfo<'x>>>,
    pub cached_reduction: Option<ConstructId<'x>>,
    pub shown_from: Vec<ConstructId<'x>>,
}

pub type VariableId<'x> = Id<Variable<'x>, 'V'>;
#[derive(Clone, Debug)]
pub struct Variable<'x> {
    pub pd: PhantomData<&'x ()>,
}
