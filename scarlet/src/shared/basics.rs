use std::fmt::{self, Debug, Formatter};

pub type Definitions = Vec<(String, ItemId)>;
pub type Replacements = Vec<(ItemId, ItemId)>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Bool,
    I32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveValue {
    Bool(bool),
    I32(i32),
}

impl PrimitiveValue {
    pub fn expect_bool(&self) -> bool {
        match self {
            Self::Bool(v) => *v,
            _ => panic!("Expected a bool"),
        }
    }

    pub fn expect_i32(&self) -> i32 {
        match self {
            Self::I32(v) => *v,
            _ => panic!("Expected an i32"),
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Self::I32(v) => Some(*v),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemId(pub(crate) usize);

impl Debug for ItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "id{{{}}}", self.0)
    }
}
