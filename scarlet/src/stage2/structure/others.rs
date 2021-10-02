use std::fmt::{self, Debug};

use super::{Namespace, Value};
use crate::shared::{Id, OrderedMap};

pub type Replacements = Vec<(ValueId, ValueId)>;
pub type Definitions = OrderedMap<String, Item>;

pub type NamespaceId = Id<Option<Namespace>>;
pub type ReplacementsId = Id<Replacements>;
pub type ValueId = Id<Option<Value>>;
pub type VariableId = Id<Variable>;
pub type VariantId = Id<Variant>;

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
