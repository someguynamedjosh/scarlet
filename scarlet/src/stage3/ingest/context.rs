use std::collections::HashMap;

use crate::{stage2::structure as s2, stage3::structure as s3};

pub struct Context<'a> {
    pub input: &'a mut s2::Environment,
    pub output: &'a mut s3::Environment,
    pub value_map: HashMap<s2::ValueId, s3::ValueId>,
    pub variable_map: HashMap<s2::VariableId, s3::VariableId>,
    pub variant_map: HashMap<s2::VariantId, s3::VariantId>,
}

impl<'a> Context<'a> {
    pub fn new(input: &'a mut s2::Environment, output: &'a mut s3::Environment) -> Self {
        Self {
            input,
            output,
            value_map: HashMap::new(),
            variable_map: HashMap::new(),
            variant_map: HashMap::new(),
        }
    }

    pub fn ingest(&mut self, value_id: s2::ValueId) -> s3::ValueId {
        let value = self.input[value_id].as_ref().expect("ICE: Undefined item");
        let result = match value {
            s2::Value::Any { .. } => todo!(),
            s2::Value::BuiltinOperation(..) => todo!(),
            s2::Value::BuiltinValue(value) => {
                let value = s3::Value::BuiltinValue(*value);
                self.output.values.get_or_push(value)
            }
            s2::Value::From { base, values } => todo!(),
            s2::Value::Identifier { name, in_namespace } => {
                let (name, in_namespace) = (name.clone(), *in_namespace);
                let item = self
                    .lookup_identifier(name, in_namespace)
                    .expect("TODO: Nice error");
                self.ingest(item.value)
            }
            s2::Value::Member { base, name, .. } => {
                let (base, name) = (*base, name.clone());
                let item = self.lookup_member(base, name).expect("TODO: Nice error");
                self.ingest(item.value)
            }
            s2::Value::Replacing { base, replacements } => todo!(),
            s2::Value::Variant { variant } => todo!(),
        };
        self.value_map.insert(value_id, result);
        result
    }

    fn lookup_identifier(
        &mut self,
        name: String,
        in_namespace: s2::NamespaceId,
    ) -> Option<s2::Item> {
        let namespace = self.input[in_namespace]
            .as_ref()
            .expect("ICE: Undefined item");
        match namespace {
            s2::Namespace::Defining {
                definitions,
                parent,
                ..
            } => {
                for (candidate, item) in definitions {
                    if candidate == &name {
                        return Some(*item);
                    }
                }
                let parent = *parent;
                self.lookup_identifier(name, parent)
            }
            s2::Namespace::Empty => None,
            s2::Namespace::Identifier {
                name: other_name,
                in_namespace: other_namespace,
            } => {
                let (other_name, other_namespace) = (other_name.clone(), *other_namespace);
                let item = self.lookup_identifier(other_name, other_namespace)?;
                self.lookup_identifier(name, item.namespace)
            }
            s2::Namespace::Member {
                base,
                name: other_name,
            } => {
                let (other_name, base) = (other_name.clone(), *base);
                let item = self.lookup_member(base, other_name)?;
                self.lookup_identifier(name, item.namespace)
            }
            s2::Namespace::Replacing { base, replacements } => todo!(),
            s2::Namespace::Root(..) => None,
        }
    }

    fn lookup_member(&mut self, base: s2::NamespaceId, name: String) -> Option<s2::Item> {
        let namespace = self.input[base].as_ref().expect("ICE: Undefined item");
        match namespace {
            s2::Namespace::Defining {
                base, definitions, ..
            } => {
                let mut backup = None;
                for (candidate, item) in definitions {
                    if candidate == &name {
                        backup = Some(*item);
                        break;
                    }
                }
                let base = *base;
                // Prefer items found in the base, otherwise return what was
                // found in self, if any.
                self.lookup_member(base, name).or(backup)
            }
            s2::Namespace::Empty => None,
            s2::Namespace::Identifier {
                name: other_name,
                in_namespace,
            } => {
                let (other_name, in_namespace) = (other_name.clone(), *in_namespace);
                let item = self.lookup_identifier(other_name, in_namespace)?;
                self.lookup_member(item.namespace, name)
            }
            s2::Namespace::Member {
                base: other_base,
                name: other_name,
            } => {
                let (other_name, other_base) = (other_name.clone(), *other_base);
                let item = self.lookup_member(other_base, other_name)?;
                self.lookup_member(item.namespace, name)
            }
            s2::Namespace::Replacing { base, replacements } => todo!(),
            s2::Namespace::Root(item) => {
                let base = item.namespace;
                self.lookup_member(base, name)
            }
        }
    }
}
