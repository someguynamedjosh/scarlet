use super::{context::Context, dereference::DereferencedItem};
use crate::{stage2::structure as s2, stage3::structure as s3};

impl<'a> Context<'a> {
    pub fn ingest_namespace(&mut self, namespace_id: s2::NamespaceId) -> s3::NamespaceId {
        if let Some(result) = self.namespace_map.get(&namespace_id) {
            return *result;
        }
        let (base, reps) = self.dereference_namespace(namespace_id);
        let base = self.ingest_dereferenced_base_namespace(base);
        let result = self.add_replacements_to_namespace(base, reps);
        self.namespace_map.insert(namespace_id, result);
        result
    }

    fn add_replacements_to_namespace(
        &mut self,
        base: s3::NamespaceId,
        replacements: Vec<s2::ReplacementsId>,
    ) -> s3::NamespaceId {
        if replacements.is_empty() {
            base
        } else {
            let replacements = self.ingest_replacements_list(replacements);
            let namespace = s3::Namespace::Replacing { base, replacements };
            self.output.namespaces.get_or_push(namespace)
        }
    }

    fn ingest_item(&mut self, item: s2::Item) -> s3::Item {
        let namespace = self.ingest_namespace(item.namespace);
        let value = self.ingest(item.value);
        s3::Item { namespace, value }
    }

    fn ingest_dereferenced_base_namespace(&mut self, base: s2::NamespaceId) -> s3::NamespaceId {
        let namespace = self.input[base].as_ref().expect("ICE: Undefined item");
        match namespace {
            s2::Namespace::Defining {
                base, definitions, ..
            } => {
                let (base, old_definitions) = (*base, definitions.clone());
                let base = self.ingest_namespace(base);
                let mut definitions = s3::Definitions::new();
                for (name, def) in old_definitions {
                    let def = self.ingest_item(def);
                    definitions.insert_no_replace(name, def);
                }
                if definitions.is_empty() {
                    base
                } else {
                    let namespace = s3::Namespace::Defining { base, definitions };
                    self.output.namespaces.get_or_push(namespace)
                }
            }
            s2::Namespace::Empty => self.output.namespaces.get_or_push(s3::Namespace::Empty),
            s2::Namespace::Identifier { .. } => unreachable!(),
            s2::Namespace::Member { .. } => unreachable!(),
            s2::Namespace::Replacing { .. } => unreachable!(),
            s2::Namespace::Root(item) => {
                let item = *item;
                let item = self.ingest_item(item);
                let namespace = s3::Namespace::Root(item);
                self.output.namespaces.get_or_push(namespace)
            }
        }
    }
}
