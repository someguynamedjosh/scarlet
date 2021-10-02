use super::DereferencedItem;
use crate::{
    stage2::structure as s2,
    stage3::{ingest::context::Context, structure as s3},
};

impl<'a> Context<'a> {
    pub fn dereference_member(
        &mut self,
        base: s2::NamespaceId,
        name: String,
    ) -> Option<DereferencedItem> {
        let namespace = self.input[base].as_ref().expect("ICE: Undefined item");
        match namespace {
            s2::Namespace::Defining {
                base, definitions, ..
            } => {
                let definitions = definitions.clone();
                let base = *base;
                self.lookup_in_defining(definitions, name, base)
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
                self.lookup_then_replace(base, name, replacements)
            }
            s2::Namespace::Root(item) => {
                let base = item.namespace;
                self.dereference_member(base, name)
            }
        }
    }

    fn lookup_in_defining(
        &mut self,
        definitions: s2::Definitions,
        name: String,
        base: s2::NamespaceId,
    ) -> Option<DereferencedItem> {
        let mut backup = None;
        for (candidate, item) in definitions {
            if candidate == name {
                backup = Some(DereferencedItem::from_item(item));
                break;
            }
        }
        self.dereference_member(base, name).or(backup)
    }

    fn lookup_then_replace(
        &mut self,
        base: s2::NamespaceId,
        name: String,
        replacements: s2::ReplacementsId,
    ) -> Option<DereferencedItem> {
        let mut result = self.dereference_member(base, name)?;
        result.replacements.push(replacements);
        Some(result)
    }
}
