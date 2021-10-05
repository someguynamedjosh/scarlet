use crate::{
    shared::{Id, OrderedMap, OrderedSet},
    stage2::{
        self,
        structure::{BuiltinOperation, BuiltinValue},
    },
};

pub type Substitution = (VariableId, ValueId);
pub type Variables = OrderedSet<VariableId>;

pub type ValueId = Id<AnnotatedValue, 'L'>;
pub type VariableId = Id<Variable, 'V'>;
pub type VariantId = Id<Variant, 'T'>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Any {
        id: VariableId,
        typee: ValueId,
    },
    BuiltinOperation(BuiltinOperation<ValueId>),
    BuiltinValue(BuiltinValue),
    From {
        base: ValueId,
        variable: VariableId,
    },
    Substituting {
        base: ValueId,
        target: VariableId,
        value: ValueId,
    },
    Variant(VariantId),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnnotatedValue {
    pub cached_type: Option<ValueId>,
    pub cached_reduction: Option<ValueId>,
    pub defined_at: Option<stage2::structure::ItemId>,
    pub value: Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub stage2_id: crate::stage2::structure::VariableId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub typee: ValueId,
}
