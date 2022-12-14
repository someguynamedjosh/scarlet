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
    environment::Environment,
    item::{CddContext, CycleDetectingDebug, ItemDefinition, ItemRef},
    shared::OrderedMap,
    util::PtrExtension,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnresolvedTarget {
    Positional,
    Named(String),
}

pub type Substitutions<Definition, Analysis> =
    Vec<(UnresolvedTarget, ItemRef<Definition, Analysis>)>;

#[derive(Clone)]
pub struct DUnresolvedSubstitution<Definition, Analysis> {
    base: ItemRef<Definition, Analysis>,
    substitutions: Substitutions<Definition, Analysis>,
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> CycleDetectingDebug
    for DUnresolvedSubstitution<Definition, Analysis>
{
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.base.fmt(f, ctx)?;
        write!(f, "(")?;
        let mut first = true;
        for (target, value) in &self.substitutions {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{:?} IS ", target)?;
            value.fmt(f, ctx)?;
        }
        write!(f, ")")
    }
}

impl<Defn: ItemDefinition<Defn, Analysis>, Analysis> ItemDefinition<Defn, Analysis>
    for DUnresolvedSubstitution<Defn, Analysis>
{
    fn children(&self) -> Vec<ItemRef<Defn, Analysis>> {
        todo!()
    }
}

impl<Definition, Analysis> DUnresolvedSubstitution<Definition, Analysis> {
    pub fn new_unresolved(
        base: ItemRef<Definition, Analysis>,
        substitutions: Substitutions<Definition, Analysis>,
    ) -> Self {
        Self {
            base,
            substitutions,
        }
    }
}
