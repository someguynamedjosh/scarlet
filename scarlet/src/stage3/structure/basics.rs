use crate::{
    shared::{Id, OrderedMap},
    stage2::structure::{BuiltinOperation, BuiltinValue},
};

pub type Substitutions = OrderedMap<VariableId, ValueId>;

pub type ValueId = Id<Value, 'L'>;
pub type VariableId = Id<Variable, 'V'>;
pub type VariantId = Id<Variant, 'T'>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Any(VariableId),
    BuiltinOperation(BuiltinOperation<ValueId>),
    BuiltinValue(BuiltinValue),
    From {
        base: ValueId,
        values: Vec<ValueId>,
    },
    Substituting {
        base: ValueId,
        substitutions: Substitutions,
    },
    Variant(VariantId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub typee: ValueId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub typee: ValueId,
}
