use super::Value;
use crate::shared::{Id, OrderedMap, OrderedSet};

pub type Definitions = OrderedMap<VariableId, ValueId>;
pub type Replacements = OrderedMap<VariableId, ValueId>;
pub type Variables = OrderedSet<(VariableId, ())>;
pub type ReplacementsId = Id<Replacements>;
pub type ValueId = Id<Value>;
pub type VariableId = Id<Variable>;
pub type VariantId = Id<Variant>;

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
