use std::collections::HashMap;

use crate::{stage2::structure as s2, stage3::structure as s3};

pub struct Context<'a> {
    pub input: &'a mut s2::Environment,
    pub output: &'a mut s3::Environment,
    pub value_map: HashMap<s2::ValueId, s3::ValueId>,
    pub variable_map: HashMap<s2::VariableId, s3::VariableId>,
    pub variant_map: HashMap<s2::VariantId, s3::VariantId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DereferencedItem {
    namespace: Option<s2::NamespaceId>,
    replacements: Vec<s2::ReplacementsId>,
    value: s2::ValueId,
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
        if let Some(result) = self.value_map.get(&value_id) {
            return *result;
        }
        let dereferenced = self.dereference_value(value_id);
        let result = self.ingest_dereferenced(dereferenced);
        self.value_map.insert(value_id, result);
        result
    }

    fn ingest_dereferenced_base(&mut self, base: s2::ValueId) -> s3::ValueId {
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
            s2::Value::From { base, values } => todo!(),
            s2::Value::Identifier { .. } => unreachable!(),
            s2::Value::Member { .. } => unreachable!(),
            s2::Value::Replacing { .. } => unreachable!(),
            s2::Value::Variant { variant } => todo!(),
        }
    }

    fn ingest_replacements(&mut self, replacements: s2::ReplacementsId) -> s3::ReplacementsId {
        let replacements = s3::Replacements::new();
        self.output.replacements.get_or_push(replacements)
    }

    fn ingest_replacements_list(
        &mut self,
        replacements: Vec<s2::ReplacementsId>,
    ) -> Vec<s3::ReplacementsId> {
        replacements
            .into_iter()
            .map(|reps| self.ingest_replacements(reps))
            .collect()
    }

    fn ingest_dereferenced(&mut self, dereferenced: DereferencedItem) -> s3::ValueId {
        let base = self.ingest_dereferenced_base(dereferenced.value);
        if dereferenced.replacements.is_empty() {
            base
        } else {
            let replacements = self.ingest_replacements_list(dereferenced.replacements);
            let value = s3::Value::Replacing { base, replacements };
            self.output.values.get_or_push(value)
        }
    }

    fn ingest_variable(&mut self, variable_id: s2::VariableId) -> s3::VariableId {
        if let Some(result) = self.variable_map.get(&variable_id) {
            return *result;
        }
        let original_type = self.input[variable_id].original_type;
        let ingested_type = self.ingest(original_type);
        // For now, set the definition to the type because we do not yet have an
        // ID for the actual definition.
        let variable = s3::Variable {
            definition: ingested_type,
            original_type: ingested_type,
        };
        let result = self.output.variables.push(variable);
        self.variable_map.insert(variable_id, result);
        result
    }

    fn dereference_value(&mut self, value_id: s2::ValueId) -> DereferencedItem {
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

    fn dereference_identifier(
        &mut self,
        name: String,
        in_namespace: s2::NamespaceId,
    ) -> Option<DereferencedItem> {
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
                        return Some(DereferencedItem::from_item(*item));
                    }
                }
                let parent = *parent;
                self.dereference_identifier(name, parent)
            }
            s2::Namespace::Empty => None,
            s2::Namespace::Identifier {
                name: other_name,
                in_namespace: other_namespace,
            } => {
                let (other_name, other_namespace) = (other_name.clone(), *other_namespace);
                let item = self.dereference_identifier(other_name, other_namespace)?;
                self.dereference_identifier(name, item.namespace.unwrap())
            }
            s2::Namespace::Member {
                base,
                name: other_name,
            } => {
                let (other_name, base) = (other_name.clone(), *base);
                let item = self.dereference_member(base, other_name)?;
                self.dereference_identifier(name, item.namespace.unwrap())
            }
            s2::Namespace::Replacing { base, replacements } => {
                unreachable!("i think? question mark?")
            }
            s2::Namespace::Root(..) => None,
        }
    }

    fn dereference_member(
        &mut self,
        base: s2::NamespaceId,
        name: String,
    ) -> Option<DereferencedItem> {
        let namespace = self.input[base].as_ref().expect("ICE: Undefined item");
        match namespace {
            s2::Namespace::Defining {
                base, definitions, ..
            } => {
                let mut backup = None;
                for (candidate, item) in definitions {
                    if candidate == &name {
                        backup = Some(DereferencedItem::from_item(*item));
                        break;
                    }
                }
                let base = *base;
                // Prefer items found in the base, otherwise return what was
                // found in self, if any.
                self.dereference_member(base, name).or(backup)
            }
            s2::Namespace::Empty => None,
            s2::Namespace::Identifier {
                name: other_name,
                in_namespace,
            } => {
                let (other_name, in_namespace) = (other_name.clone(), *in_namespace);
                let item = self.dereference_identifier(other_name, in_namespace)?;
                self.dereference_member(item.namespace.unwrap(), name)
            }
            s2::Namespace::Member {
                base: other_base,
                name: other_name,
            } => {
                let (other_name, other_base) = (other_name.clone(), *other_base);
                let item = self.dereference_member(other_base, other_name)?;
                self.dereference_member(item.namespace.unwrap(), name)
            }
            s2::Namespace::Replacing { base, replacements } => {
                let (base, replacements) = (*base, *replacements);
                let mut result = self.dereference_member(base, name)?;
                result.replacements.push(replacements);
                Some(result)
            }
            s2::Namespace::Root(item) => {
                let base = item.namespace;
                self.dereference_member(base, name)
            }
        }
    }
}
