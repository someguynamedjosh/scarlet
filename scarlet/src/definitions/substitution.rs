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
    environment::{Environment, ItemId},
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

pub type Substitutions = OrderedMap<(ItemId, ParameterPtr), ItemId>;

#[derive(Clone, Debug)]
pub struct DResolvedSubstitution {
    base: ItemId,
    substitutions: Substitutions,
}

impl DResolvedSubstitution {
    pub fn new(base: ItemId, substitutions: Substitutions) -> Self {
        Self {
            base,
            substitutions,
        }
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.substitutions
    }
}
