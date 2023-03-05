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
pub type Substitutions = OrderedMap<(ItemId, ParameterPtr), ItemId>;

#[derive(Clone, Debug)]
pub struct DSubstitution {
    base: ItemId,
    substitutions: Result<Substitutions, UnresolvedSubstitutions>,
}

impl DSubstitution {
    pub fn new_unresolved(base: ItemId, substitutions: UnresolvedSubstitutions) -> Self {
        Self {
            base,
            substitutions: Err(substitutions),
        }
    }

    pub fn new_resolved(base: ItemId, substitutions: Substitutions) -> Self {
        Self {
            base,
            substitutions: Ok(substitutions),
        }
    }
}
