use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};

mod identifier;
mod member;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DereferencedItem {
    pub namespace: Option<s2::NamespaceId>,
    pub replacements: Vec<s2::ReplacementsId>,
    pub value: s2::ValueId,
}

impl DereferencedItem {
    fn from_item(item: s2::Item) -> Self {
        Self {
            namespace: Some(item.namespace),
            replacements: Vec::new(),
            value: item.value,
        }
    }
}

impl<'a> Context<'a> {
    pub fn dereference_variable(&mut self, value_id: s2::ValueId) -> s3::VariableId {
        let dereferenced = self.dereference_value(value_id);
        if !dereferenced.replacements.is_empty() {
            todo!("nice error")
        }
        let value = self.input[dereferenced.value]
            .as_ref()
            .expect("ICE: Undefined item");
        if let s2::Value::Any { variable } = value {
            let variable = *variable;
            self.ingest_variable(variable)
        } else {
            todo!("Nice error")
        }
    }

    pub fn dereference_value(&mut self, value_id: s2::ValueId) -> DereferencedItem {
        let value = self.input[value_id].as_ref().expect("ICE: Undefined item");
        match value {
            s2::Value::Identifier { name, in_namespace } => {
                let (name, in_namespace) = (name.clone(), *in_namespace);
                self.dereference_identifier(name, in_namespace)
                    .expect("TODO: Nice error")
            }
            s2::Value::Member { base, name, .. } => {
                let (base, name) = (*base, name.clone());
                self.dereference_member(base, name)
                    .expect("TODO: Nice error")
            }
            s2::Value::Replacing { base, replacements } => {
                let (base, replacements) = (*base, *replacements);
                let mut base = self.dereference_value(base);
                base.replacements.push(replacements);
                base
            }
            _ => DereferencedItem {
                namespace: None,
                replacements: Vec::new(),
                value: value_id,
            },
        }
    }
}
