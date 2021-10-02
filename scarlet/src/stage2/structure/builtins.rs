#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinOperation<ValueId> {
    Cast {
        equality_proof: ValueId,
        original_type: ValueId,
        new_type: ValueId,
        original_value: ValueId,
    },
}

impl<ValueId: Copy> BuiltinOperation<ValueId> {
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

    pub fn with_inputs<V: Copy>(&self, new_inputs: Vec<V>) -> BuiltinOperation<V> {
        match self {
            Self::Cast { .. } => {
                assert_eq!(new_inputs.len(), 4);
                BuiltinOperation::Cast {
                    equality_proof: new_inputs[0],
                    original_type: new_inputs[1],
                    new_type: new_inputs[2],
                    original_value: new_inputs[3],
                }
            }
        }
    }

    pub fn map<V: Copy>(&self, f: impl FnMut(ValueId) -> V) -> BuiltinOperation<V> {
        self.with_inputs(self.inputs().into_iter().map(f).collect())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuiltinValue {
    OriginType,
    U8Type,
    U8(u8),
}
