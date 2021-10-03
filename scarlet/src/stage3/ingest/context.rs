use std::collections::HashMap;

use crate::{stage2::structure as s2, stage3::structure as s3};

pub struct Context<'a> {
    pub input: &'a s2::Environment,
    pub output: &'a mut s3::Environment,
    pub namespace_map: HashMap<s2::NamespaceId, s3::NamespaceId>,
    pub value_map: HashMap<s2::ValueId, s3::ValueId>,
    pub variable_map: HashMap<s2::VariableId, s3::VariableId>,
    pub variant_map: HashMap<s2::VariantId, s3::VariantId>,
}

impl<'a> Context<'a> {
    pub fn new(input: &'a s2::Environment, output: &'a mut s3::Environment) -> Self {
        Self {
            input,
            output,
            namespace_map: HashMap::new(),
            value_map: HashMap::new(),
            variable_map: HashMap::new(),
            variant_map: HashMap::new(),
        }
    }
}
