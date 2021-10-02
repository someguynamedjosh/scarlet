use super::Value;
use crate::shared::{Id, OrderedMap, OrderedSet};

pub type BuiltinOperation = crate::stage2::structure::BuiltinOperation<ValueId>;
pub type Definitions = OrderedMap<VariableId, ValueId>;
pub type Replacements = OrderedMap<VariableId, ValueId>;
pub type Variables = OrderedSet<VariableId>;

pub type ReplacementsId = Id<Replacements, 'R'>;
pub type ValueId = Id<Value, 'L'>;
pub type VariableId = Id<Variable, 'V'>;
pub type VariantId = Id<Variant, 'T'>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub definition: ValueId,
    pub original_type: ValueId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub definition: ValueId,
    pub original_type: ValueId,
}
