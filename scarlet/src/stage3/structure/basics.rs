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
    pub cached_type: Option<ValueId>,
    pub cached_reduction: Option<ValueId>,
    pub defined_at: OrderedSet<stage2::structure::ItemId>,
    pub referenced_at: OrderedSet<stage2::structure::ItemId>,
    pub display_requested_from: OrderedSet<stage2::structure::ItemId>,
    pub value: Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct OpaqueValue {
    pub stage2_id: crate::stage2::structure::OpaqueId,
}
