use std::fmt::{self, Debug, Formatter};

use super::{ItemId, PrimitiveValue};

#[derive(Clone, PartialEq, Eq, Hash)]
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

    pub fn with_inputs(&self, new_inputs: Vec<ItemId>) -> Self {
        match self {
            Self::Sum(..) => {
                assert_eq!(new_inputs.len(), 2);
                Self::Sum(new_inputs[0], new_inputs[1])
            }
            Self::Difference(..) => {
                assert_eq!(new_inputs.len(), 2);
                Self::Difference(new_inputs[0], new_inputs[1])
            }
        }
    }
}

impl Debug for IntegerMathOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sum(l, r) => write!(f, "sum[{:?} {:?}]", l, r),
            Self::Difference(l, r) => write!(f, "difference[{:?} {:?}]", l, r),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveOperation {
    I32Math(IntegerMathOperation),
}

impl PrimitiveOperation {
    pub fn inputs(&self) -> Vec<ItemId> {
        match self {
            Self::I32Math(op) => op.inputs(),
        }
    }

    pub fn with_inputs(&self, new_inputs: Vec<ItemId>) -> Self {
        match self {
            Self::I32Math(op) => Self::I32Math(op.with_inputs(new_inputs)),
        }
    }

    pub fn compute(&self, inputs: Vec<PrimitiveValue>) -> PrimitiveValue {
        use IntegerMathOperation as Imo;
        match self {
            Self::I32Math(op) => {
                let inputs: Vec<_> = inputs.iter().map(PrimitiveValue::expect_i32).collect();
                match op {
                    Imo::Sum(..) => PrimitiveValue::I32(inputs[0] + inputs[1]),
                    Imo::Difference(..) => PrimitiveValue::I32(inputs[0] - inputs[1]),
                }
            }
        }
    }
}

impl Debug for PrimitiveOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::I32Math(op) => write!(f, "Integer32::{:?}", op),
        }
    }
}
