use std::fmt::{self, Debug, Formatter};

use super::ItemId;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerMathOperation {
    Sum(ItemId, ItemId),
    Difference(ItemId, ItemId),
    /* Multiply(ItemId, ItemId),
     * IntegerDivide(ItemId, ItemId),
     * Modulo(ItemId, ItemId),
     * Negate(ItemId), */
}

impl IntegerMathOperation {
    pub fn inputs(&self) -> Vec<ItemId> {
        match self {
            Self::Sum(a, b) | Self::Difference(a, b) => vec![*a, *b],
        }
    }
}

impl Debug for IntegerMathOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sum(l, r) => write!(f, "sum {:?} {:?}}}", l, r),
            Self::Difference(l, r) => write!(f, "difference {:?} {:?}}}", l, r),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinOperation {
    I32Math(IntegerMathOperation),
    AreSameVariant {
        base: ItemId,
        other: ItemId,
    },
    Reinterpret {
        proof_equal: ItemId,
        original_type: ItemId,
        new_type: ItemId,
        original: ItemId,
    },
}

impl BuiltinOperation {
    pub fn inputs(&self) -> Vec<ItemId> {
        match self {
            Self::I32Math(op) => op.inputs(),
            Self::AreSameVariant { base, other } => vec![*base, *other],
            Self::Reinterpret {
                proof_equal,
                original_type,
                new_type,
                original,
            } => vec![*proof_equal, *original_type, *new_type, *original],
        }
    }
}

impl Debug for BuiltinOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::I32Math(op) => write!(f, "builtin_item{{i32_{:?}", op),
            Self::AreSameVariant { base, other } => {
                write!(f, "builtin_item{{are_same_variant {:?} {:?}}}", base, other)
            }
            Self::Reinterpret {
                proof_equal,
                original_type,
                new_type,
                original,
            } => {
                write!(
                    f,
                    "builtin_item{{reinterpret {:?} {:?} {:?} {:?}}}",
                    proof_equal, original_type, new_type, original
                )
            }
        }
    }
}
