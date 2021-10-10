use serde::Serialize;

use crate::{
    shared::{Id, OpaqueClass, OrderedMap, OrderedSet},
    stage2::{
        self,
        structure::{BuiltinOperation, BuiltinValue},
    },
};

pub type Substitutions = Vec<(Option<ValueId>, ValueId)>;
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
    Substituting {
        base: ValueId,
        substitutions: Substitutions,
    },
    TypeIs {
        base: ValueId,
        typee: ValueId,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct AnnotatedValue {
    pub value: Option<Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct OpaqueValue {
    pub stage2_id: crate::stage2::structure::OpaqueId,
}
