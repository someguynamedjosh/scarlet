use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};

mod identifier;
mod member;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DereferencedItem {
    pub namespace: s2::NamespaceId,
    pub replacements: Vec<s2::ReplacementsId>,
    pub value: s2::ValueId,
}

impl DereferencedItem {
    fn from_item(item: s2::Item) -> Self {
        Self {
            namespace: item.namespace,
            replacements: Vec::new(),
            value: item.value,
        }
    }

    pub fn including_replacements(self, from: Self) -> Self {
        Self {
            replacements: [self.replacements, from.replacements].concat(),
            ..self
        }
    }
}

impl<'a> Context<'a> {
    pub fn dereference_variable(&mut self, value_id: s2::ValueId) -> s3::VariableId {
        let (value, replacements) = self.dereference_value(value_id);
        if !replacements.is_empty() {
            todo!("nice error")
        }
        let value = self.input[value].as_ref().expect("ICE: Undefined item");
        if let s2::Value::Any { variable } = value {
            let variable = *variable;
            self.ingest_variable(variable)
        } else {
            todo!("Nice error")
        }
    }

    pub fn dereference_value(
        &mut self,
        value_id: s2::ValueId,
    ) -> (s2::ValueId, Vec<s2::ReplacementsId>) {
        let value = self.input[value_id].as_ref().expect("ICE: Undefined item");
        match value {
            s2::Value::Identifier { name, in_namespace } => {
                let (name, in_namespace) = (name.clone(), *in_namespace);
                let mut next_item = self
                    .dereference_identifier(name, in_namespace)
                    .expect("TODO: Nice error");
                let (value, mut reps) = self.dereference_value(next_item.value);
                reps.append(&mut next_item.replacements);
                (value, reps)
            }
            s2::Value::Member { base, name, .. } => {
                let (base, name) = (*base, name.clone());
                let mut next_item = self
                    .dereference_member(base, name)
                    .expect("TODO: Nice error");
                let (value, mut reps) = self.dereference_value(next_item.value);
                reps.append(&mut next_item.replacements);
                (value, reps)
            }
            s2::Value::Replacing { base, replacements } => {
                let (base, replacements) = (*base, *replacements);
                let (value, mut reps) = self.dereference_value(base);
                reps.push(replacements);
                (value, reps)
            }
            _ => (value_id, Vec::new()),
        }
    }

    pub fn dereference_namespace(
        &mut self,
        namespace_id: s2::NamespaceId,
    ) -> (s2::NamespaceId, Vec<s2::ReplacementsId>) {
        let namespace = self.input[namespace_id]
            .as_ref()
            .expect("ICE: Undefined item");
        match namespace {
            s2::Namespace::Identifier { name, in_namespace } => {
                let (name, in_namespace) = (name.clone(), *in_namespace);
                let mut next_item = self
                    .dereference_identifier(name, in_namespace)
                    .expect("TODO: Nice error");
                let (namespace, mut reps) = self.dereference_namespace(next_item.namespace);
                reps.append(&mut next_item.replacements);
                (namespace, reps)
            }
            s2::Namespace::Member { base, name, .. } => {
                let (base, name) = (*base, name.clone());
                let mut next_item = self
                    .dereference_member(base, name)
                    .expect("TODO: Nice error");
                let (namespace, mut reps) = self.dereference_namespace(next_item.namespace);
                reps.append(&mut next_item.replacements);
                (namespace, reps)
            }
            s2::Namespace::Replacing { base, replacements } => {
                let (base, replacements) = (*base, *replacements);
                let (namespace, mut reps) = self.dereference_namespace(base);
                reps.push(replacements);
                (namespace, reps)
            }
            _ => (namespace_id, Vec::new()),
        }
    }
}
