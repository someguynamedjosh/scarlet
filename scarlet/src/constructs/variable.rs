use super::base::{Construct, ConstructDefinition, ConstructId};
use crate::{
    impl_any_eq_for_construct,
    shared::{Id, Pool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VarType {
    Anything,
    _32U,
    Bool,
    Just(ConstructId),
    And(ConstructId, ConstructId),
    Or(ConstructId, ConstructId),
    Array {
        length: ConstructId,
        eltype: ConstructId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable;
pub type VariablePool = Pool<Variable, 'V'>;
pub type VariableId = Id<'V'>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CVariable {
    pub id: VariableId,
    pub typee: VarType,
    pub capturing: bool,
}

impl_any_eq_for_construct!(CVariable);

impl Construct for CVariable {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }
}
