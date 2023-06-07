use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;

use super::{
    builtin::DBuiltin,
    parameter::{DParameter, ParameterPtr},
};
use crate::{
    definitions::{identifier::DIdentifier, member_access::DMemberAccess},
    diagnostic::Diagnostic,
    environment::{Env3, Environment, ItemId},
    shared::OrderedMap,
    util::PtrExtension,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnresolvedTarget {
    Positional,
    Named(String),
}

pub type UnresolvedSubstitutions = Vec<(UnresolvedTarget, ItemId)>;

#[derive(Clone, Debug)]
pub struct DUnresolvedSubstitution {
    base: ItemId,
    substitutions: UnresolvedSubstitutions,
}

impl DUnresolvedSubstitution {
    pub fn new(base: ItemId, substitutions: UnresolvedSubstitutions) -> Self {
        Self {
            base,
            substitutions,
        }
    }

    pub fn substitutions(&self) -> &UnresolvedSubstitutions {
        &self.substitutions
    }

    pub fn base(&self) -> ItemId {
        self.base
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PartiallyResolvedTarget {
    Positional,
    Item(ItemId),
}

pub type PartiallyResolvedSubstitutions = OrderedMap<PartiallyResolvedTarget, ItemId>;

#[derive(Clone, Debug)]
pub struct DPartiallyResolvedSubstitution {
    base: ItemId,
    substitutions: PartiallyResolvedSubstitutions,
}

impl DPartiallyResolvedSubstitution {
    pub fn new(base: ItemId, substitutions: PartiallyResolvedSubstitutions) -> Self {
        Self {
            base,
            substitutions,
        }
    }

    pub fn substitutions(&self) -> &PartiallyResolvedSubstitutions {
        &self.substitutions
    }

    pub fn base(&self) -> ItemId {
        self.base
    }
}

pub type Substitutions = OrderedMap<ParameterPtr, ItemId>;

#[derive(Clone, Debug)]
pub struct DSubstitution {
    base: ItemId,
    substitutions: Substitutions,
    do_asserts: bool,
}

impl DSubstitution {
    pub fn new(base: ItemId, substitutions: Substitutions) -> Self {
        Self {
            base,
            substitutions,
            do_asserts: true,
        }
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.substitutions
    }

    pub fn base(&self) -> ItemId {
        self.base
    }

    pub fn add_type_asserts(&self, env: &mut Env3) {
        if !self.do_asserts {
            return;
        }
        for (target, value) in &self.substitutions {
            let target_type = target.original_type();
            let target_type_deps = env.get_deps(target_type);
            let target_type_subs: Substitutions = self
                .substitutions
                .iter()
                .filter(|(k, _)| target_type_deps.contains(k))
                .cloned()
                .collect();
            if target_type_subs.is_empty() {
                env.assert_of_type(*value, target_type);
            } else {
                let subbed_target_type = env.new_defined_item(Self {
                    base: target_type,
                    substitutions: target_type_subs,
                    do_asserts: false,
                });
                env.assert_of_type(*value, subbed_target_type);
            }
        }
    }
}
