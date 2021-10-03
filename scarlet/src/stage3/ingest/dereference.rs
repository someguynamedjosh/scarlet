use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};

#[derive(Clone, Debug)]
pub struct ItemBeingDereferenced {
    pub namespace: DereferencedNamespace,
    pub value: DereferencedValue,
}

#[derive(Clone, Debug)]
pub struct DereferencedNamespace {
    pub id: s2::NamespaceId,
    pub replacements: Vec<s2::ReplacementsId>,
}

#[derive(Clone, Debug)]
pub struct DereferencedValue {
    pub id: s2::ValueId,
    pub replacements: Vec<s2::ReplacementsId>,
}

impl ItemBeingDereferenced {
    pub fn from(item: s2::Item) -> Self {
        Self {
            namespace: DereferencedNamespace {
                id: item.namespace,
                replacements: Vec::new(),
            },
            value: DereferencedValue {
                id: item.value,
                replacements: Vec::new(),
            },
        }
    }

    pub fn as_base_of(mut self, mut other: ItemBeingDereferenced) -> Self {
        self.namespace
            .replacements
            .append(&mut other.namespace.replacements);
        self.value
            .replacements
            .append(&mut other.value.replacements);
        self
    }
}

impl<'a> Context<'a> {
    pub fn dereference(&mut self, item: ItemBeingDereferenced) -> ItemBeingDereferenced {
        let namespace = self.dereference_namespace(item.namespace.id);
        let value = self.dereference_value(item.value.id);
        ItemBeingDereferenced { namespace, value }.as_base_of(item)
    }

    pub fn dereference_namespace(&mut self, namespace_id: s2::NamespaceId) -> DereferencedNamespace {
        let namespace = self.input[namespace_id].as_ref();
        let namespace = namespace.expect("ICE: Undefined item");
        match namespace {
            s2::Namespace::Identifier { name, in_namespace } => {
                let (name, in_namespace) = (name.clone(), *in_namespace);
                let identified = self
                    .lookup_identifier(name, in_namespace)
                    .expect("TODO: Nice error");
                identified.namespace
            }
            s2::Namespace::Member { base, name } => {
                let (base, name) = (*base, name.clone());
                let member = self.lookup_member(base, name).expect("TODO: Nice error");
                member.namespace
            }
            s2::Namespace::Replacing { base, replacements } => {
                let (base, replacements) = (*base, *replacements);
                let mut dereffed = self.dereference_namespace(base);
                dereffed.replacements.push(replacements);
                dereffed
            }
            _ => DereferencedNamespace {
                id: namespace_id,
                replacements: vec![],
            },
        }
    }

    pub fn dereference_value(&mut self, value_id: s2::ValueId) -> DereferencedValue {
        let value = self.input[value_id].as_ref();
        let value = value.expect("ICE: Undefined item");
        match value {
            s2::Value::Identifier { name, in_namespace } => {
                let (name, in_namespace) = (name.clone(), *in_namespace);
                let identified = self
                    .lookup_identifier(name, in_namespace)
                    .expect("TODO: Nice error");
                identified.value
            }
            s2::Value::Member { base, name, .. } => {
                let (base, name) = (*base, name.clone());
                let member = self.lookup_member(base, name).expect("TODO: Nice error");
                member.value
            }
            s2::Value::Replacing { base, replacements } => {
                let (base, replacements) = (*base, *replacements);
                let mut dereffed = self.dereference_value(base);
                dereffed.replacements.push(replacements);
                dereffed
            }
            _ => DereferencedValue {
                id: value_id,
                replacements: vec![],
            },
        }
    }

    fn lookup_identifier(
        &mut self,
        name: String,
        in_namespace: s2::NamespaceId,
    ) -> Option<ItemBeingDereferenced> {
        let dereffed = self.dereference_namespace(in_namespace);
        let namespace = self.input[dereffed.id].as_ref();
        let namespace = namespace.expect("ICE: Undefined item");
        let item = match namespace {
            s2::Namespace::Defining {
                definitions,
                parent,
                ..
            } => {
                let mut retval = None;
                for (candidate, item) in definitions {
                    if candidate == &name {
                        retval = Some(ItemBeingDereferenced::from(*item));
                    }
                }
                if let Some(retval) = retval {
                    Some(retval)
                } else {
                    let parent = *parent;
                    let mut item = self.lookup_identifier(name, parent)?;
                    item.namespace
                        .replacements
                        .append(&mut dereffed.replacements.clone());
                    item.value
                        .replacements
                        .append(&mut dereffed.replacements.clone());
                    Some(item)
                }
            }
            s2::Namespace::Empty => None,
            s2::Namespace::Identifier { .. } => unreachable!(),
            s2::Namespace::Member { .. } => unreachable!(),
            s2::Namespace::Replacing { .. } => unreachable!(),
            s2::Namespace::Root(..) => None,
        }?;
        Some(self.dereference(item))
    }

    fn lookup_member(
        &mut self,
        base: s2::NamespaceId,
        name: String,
    ) -> Option<ItemBeingDereferenced> {
        let dereffed = self.dereference_namespace(base);
        let namespace = self.input[dereffed.id].as_ref();
        let namespace = namespace.expect("ICE: Undefined item");
        let item = match namespace {
            s2::Namespace::Defining {
                definitions, base, ..
            } => {
                let mut backup = None;
                for (candidate, item) in definitions {
                    if candidate == &name {
                        backup = Some(ItemBeingDereferenced::from(*item));
                    }
                }
                let base = *base;
                let mut item = self.lookup_member(base, name).or(backup)?;
                item.namespace
                    .replacements
                    .append(&mut dereffed.replacements.clone());
                item.value
                    .replacements
                    .append(&mut dereffed.replacements.clone());
                Some(item)
            }
            s2::Namespace::Empty => None,
            s2::Namespace::Identifier { .. } => unreachable!(),
            s2::Namespace::Member { .. } => unreachable!(),
            s2::Namespace::Replacing { .. } => unreachable!(),
            s2::Namespace::Root(..) => None,
        }?;
        Some(self.dereference(item))
    }
}
