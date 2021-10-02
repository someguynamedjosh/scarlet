use super::DereferencedItem;
use crate::{
    stage2::structure as s2,
    stage3::{ingest::context::Context, structure as s3},
};

impl<'a> Context<'a> {
    pub fn dereference_identifier(
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
                let definitions = definitions.clone();
                let parent = *parent;
                lookup_in_defining(self, definitions, name, parent)
            }
            s2::Namespace::Empty => None,
            s2::Namespace::Identifier {
                name: other_name,
                in_namespace: other_namespace,
            } => {
                let (other_name, other_namespace) = (other_name.clone(), *other_namespace);
                let item = self.dereference_identifier(other_name, other_namespace)?;
                self.dereference_identifier(name, item.namespace)
            }
            s2::Namespace::Member {
                base,
                name: other_name,
            } => {
                let (other_name, base) = (other_name.clone(), *base);
                let item = self.dereference_member(base, other_name)?;
                self.dereference_identifier(name, item.namespace)
            }
            s2::Namespace::Replacing {
                base: _,
                replacements: _,
            } => {
                unreachable!("i think? question mark?")
            }
            s2::Namespace::Root(..) => None,
        }
    }
}

fn lookup_in_defining(
    this: &mut Context,
    definitions: s2::Definitions,
    name: String,
    parent: s2::NamespaceId,
) -> Option<DereferencedItem> {
    for (candidate, item) in definitions {
        if candidate == name {
            return Some(DereferencedItem::from_item(item));
        }
    }
    this.dereference_identifier(name, parent)
}
