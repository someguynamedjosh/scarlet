use super::{context::Context, dereference::DereferencedItem};
use crate::{stage2::structure as s2, stage3::structure as s3};

impl<'a> Context<'a> {
    pub fn ingest(&mut self, value_id: s2::ValueId) -> s3::ValueId {
        if let Some(result) = self.value_map.get(&value_id) {
            return *result;
        }
        let dereferenced = self.dereference_value(value_id);
        let result = self.ingest_dereferenced(dereferenced);
        self.value_map.insert(value_id, result);
        result
    }

    pub fn ingest_dereferenced_base(&mut self, base: s2::ValueId) -> s3::ValueId {
        let value = self.input[base].as_ref().expect("ICE: Undefined item");
        match value {
            s2::Value::Any { variable } => {
                let variable = *variable;
                let variable = self.ingest_variable(variable);
                let value = s3::Value::Any { variable };
                self.output.values.get_or_push(value)
            }
            s2::Value::BuiltinOperation(..) => todo!(),
            s2::Value::BuiltinValue(value) => {
                let value = s3::Value::BuiltinValue(*value);
                self.output.values.get_or_push(value)
            }
            s2::Value::From { base: _, values: _ } => todo!(),
            s2::Value::Identifier { .. } => unreachable!(),
            s2::Value::Member { .. } => unreachable!(),
            s2::Value::Replacing { .. } => unreachable!(),
            s2::Value::Variant { variant: _ } => todo!(),
        }
    }

    pub fn ingest_dereferenced(&mut self, dereferenced: DereferencedItem) -> s3::ValueId {
        let base = self.ingest_dereferenced_base(dereferenced.value);
        if dereferenced.replacements.is_empty() {
            base
        } else {
            let replacements = self.ingest_replacements_list(dereferenced.replacements);
            let value = s3::Value::Replacing { base, replacements };
            self.output.values.get_or_push(value)
        }
    }
}
