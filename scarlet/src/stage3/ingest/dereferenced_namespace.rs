use super::{context::Context, dereference::DereferencedNamespace};
use crate::{stage2::structure as s2, stage3::structure as s3};

impl<'a> Context<'a> {
    pub fn ingest_dereferenced_namespace(
        &mut self,
        dereferenced: DereferencedNamespace,
    ) -> s3::NamespaceId {
        let base = self.ingest_dereferenced_namespace_base(dereferenced.id);
        if dereferenced.replacements.is_empty() {
            base
        } else {
            let replacements = self.ingest_replacements_list(dereferenced.replacements);
            let namespace = s3::Namespace::Replacing { base, replacements };
            self.output.namespaces.get_or_push(namespace)
        }
    }

    pub fn ingest_dereferenced_namespace_base(&mut self, base: s2::NamespaceId) -> s3::NamespaceId {
        let namespace = self.input[base].as_ref().expect("ICE: Undefined item");
        let ns = match namespace {
            s2::Namespace::Defining {
                base, definitions, ..
            } => {
                let (base, definitions) = (*base, definitions.clone());
                let base = self.ingest_namespace(base);
                let mut new_definitions = s3::Definitions::new();
                for (name, item) in definitions {
                    let namespace = self.ingest_namespace(item.namespace);
                    let value = self.ingest_value(item.value);
                    let item = s3::Item { namespace, value };
                    new_definitions.insert_no_replace(name, item);
                }
                let definitions = new_definitions;
                s3::Namespace::Defining { base, definitions }
            }
            s2::Namespace::Empty => s3::Namespace::Empty,
            s2::Namespace::Identifier { .. } => unreachable!(),
            s2::Namespace::Member { .. } => unreachable!(),
            s2::Namespace::Replacing { .. } => unreachable!(),
            s2::Namespace::Root(item) => {
                let item = *item;
                let namespace = self.ingest_namespace(item.namespace);
                let value = self.ingest_value(item.value);
                let item = s3::Item { namespace, value };
                s3::Namespace::Root(item)
            }
        };
        self.output.namespaces.get_or_push(ns)
    }
}
