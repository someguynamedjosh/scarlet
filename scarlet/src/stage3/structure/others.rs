use std::fmt::{self, Debug};

use super::{Namespace, Value};
use crate::shared::{Id, OrderedMap, OrderedSet};

pub type BuiltinOperation = crate::stage2::structure::BuiltinOperation<ValueId>;
pub type Definitions = OrderedMap<String, Item>;
pub type Replacements = OrderedMap<VariableId, ValueId>;
pub type Variables = OrderedSet<VariableId>;

pub type NamespaceId = Id<Namespace, 'N'>;
pub type ReplacementsId = Id<Replacements, 'R'>;
pub type ValueId = Id<Value, 'L'>;
pub type VariableId = Id<Variable, 'V'>;
pub type VariantId = Id<Variant, 'T'>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item {
    pub namespace: NamespaceId,
    pub value: ValueId,
}

impl Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}, {:?}", self.namespace, self.value)
    }
}

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
