use super::{
    BuiltinOperation, BuiltinValue, NamespaceId, ReplacementsId, ValueId, VariableId, VariantId,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Any {
        variable: VariableId,
    },
    BuiltinOperation(BuiltinOperation),
    BuiltinValue(BuiltinValue),
    From {
        base: ValueId,
        values: Vec<ValueId>,
    },
    Identifier {
        name: String,
        in_namespace: NamespaceId,
    },
    Member {
        /// Kept for vomiting.
        previous_value: ValueId,
        base: NamespaceId,
        name: String,
    },
    Replacing {
        base: ValueId,
        replacements: ReplacementsId,
    },
    Variant {
        variant: VariantId,
    },
}
