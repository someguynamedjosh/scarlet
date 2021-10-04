use crate::{shared::{Id, OrderedMap}, stage2::structure::{BuiltinOperation, BuiltinValue}};

pub type Replacements = OrderedMap<VariableId, Value>;

pub type VariableId = Id<Variable, 'V'>;
pub type VariantId = Id<Variant, 'T'>;

pub enum ValuePath {
    
}

pub enum Value {
    Any {
        typee: Box<Value>,
    },
    BuiltinOperation(Box<BuiltinOperation<Value>>),
    BuiltinValue(BuiltinValue),
    From {
        base: Box<Value>,
        values: Vec<Value>,
    },
    Replacing {
        base: Box<Value>,
        replacements: Replacements,
    },
    Variant {
        typee: Box<Value>,
    },
}

pub struct Variable {
    typee: Value,
}

pub struct Variant {
    typee: Value,
}
