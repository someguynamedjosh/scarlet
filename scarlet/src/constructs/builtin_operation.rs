use super::base::{Construct, ConstructId};
use crate::impl_any_eq_for_construct;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CBuiltinOperation {
    pub op: BuiltinOperation,
    pub args: Vec<ConstructId>,
}

impl_any_eq_for_construct!(CBuiltinOperation);

impl Construct for CBuiltinOperation {}
