use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinValue {
    OriginType,
    BoolType,
    Bool(bool),
    I32Type,
    I32(i32),
}

impl BuiltinValue {
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
