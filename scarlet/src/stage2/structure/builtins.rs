use super::ValueId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinOperation {
    Cast {
        equality_proof: ValueId,
        original_type: ValueId,
        new_type: ValueId,
        original_value: ValueId,
    },
}

impl BuiltinOperation {
    pub fn inputs(&self) -> Vec<ValueId> {
        match self {
            Self::Cast {
                equality_proof,
                original_type,
                new_type,
                original_value,
            } => vec![*equality_proof, *original_type, *new_type, *original_value],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinValue {
    OriginType,
    U8Type,
    U8(u8),
}
