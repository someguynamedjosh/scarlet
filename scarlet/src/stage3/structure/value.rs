use super::{
    BuiltinOperation, BuiltinValue, Replacements, ReplacementsId, ValueId, VariableId, Variables,
    VariantId,
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
        variables: Variables,
    },
    Replacing {
        base: ValueId,
        replacements: Vec<ReplacementsId>,
    },
    Variant {
        variant: VariantId,
    },
}
