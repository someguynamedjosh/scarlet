use serde::Serialize;

use crate::{
    shared::{Id, OpaqueClass, OrderedMap, OrderedSet},
    stage2::structure::{BuiltinOperation, BuiltinValue},
    stage3, stage4,
};

pub type Substitutions = OrderedMap<OpaqueId, ValueId>;
pub type Variables = OrderedSet<OpaqueId>;

pub type ValueId = Id<AnnotatedValue, 'L'>;
pub type OpaqueId = Id<OpaqueValue, 'O'>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum Value {
    BuiltinOperation(BuiltinOperation<ValueId>),
    BuiltinValue(BuiltinValue),
    From {
        base: ValueId,
        value: ValueId,
    },
    Match {
        base: ValueId,
        cases: Vec<(ValueId, ValueId)>,
    },
    Opaque {
        class: OpaqueClass,
        id: OpaqueId,
        typee: ValueId,
    },
    SelfReference {
        original_id: stage4::structure::ValueId,
    },
    Substituting {
        base: ValueId,
        substitutions: Substitutions,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct AnnotatedValue {
    pub cached_type: Option<ValueId>,
    pub cached_reduction: Option<ValueId>,
    pub value: Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct OpaqueValue {
    pub stage3_id: crate::stage3::structure::OpaqueId,
}
