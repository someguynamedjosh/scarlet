use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};

impl<'a> Context<'a> {
    pub fn ingest_namespace(&mut self, namespace_id: s2::NamespaceId) -> s3::NamespaceId {
        if let Some(result) = self.namespace_map.get(&namespace_id) {
            return *result;
        }
        let dereferenced = self.dereference_namespace(namespace_id);
        let result = self.ingest_dereferenced_namespace(dereferenced);
        self.namespace_map.insert(namespace_id, result);
        result
    }

    pub fn ingest_value(&mut self, value_id: s2::ValueId) -> s3::ValueId {
        if let Some(result) = self.value_map.get(&value_id) {
            return *result;
        }
        let dereferenced = self.dereference_value(value_id);
        let result = self.ingest_dereferenced_value(dereferenced);
        self.value_map.insert(value_id, result);
        result
    }

    pub fn get_dependencies(&self, of: s3::ValueId) -> s3::Variables {
        match &self.output[of] {
            s3::Value::Any { variable } => vec![(*variable, ())].into_iter().collect(),
            s3::Value::BuiltinOperation(op) => op
                .inputs()
                .into_iter()
                .flat_map(|input| self.get_dependencies(input).into_iter())
                .collect(),
            s3::Value::BuiltinValue(..) => s3::Variables::new(),
            s3::Value::From {
                base: _,
                variables: _,
            } => todo!(),
            s3::Value::Replacing {
                base: _,
                replacements: _,
            } => todo!(),
            s3::Value::Variant { .. } => s3::Variables::new(),
        }
    }
}
