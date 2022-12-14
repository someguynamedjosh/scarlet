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

pub type UnresolvedSubstitutions<Definition, Analysis> =
    Vec<(UnresolvedTarget, ItemRef<Definition, Analysis>)>;
pub type Substitutions<Definition, Analysis> = OrderedMap<
    (
        ItemRef<Definition, Analysis>,
        ParameterPtr<Definition, Analysis>,
    ),
    ItemRef<Definition, Analysis>,
>;

#[derive(Clone)]
pub struct DSubstitution<Definition, Analysis> {
    base: ItemRef<Definition, Analysis>,
    substitutions:
        Result<Substitutions<Definition, Analysis>, UnresolvedSubstitutions<Definition, Analysis>>,
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> CycleDetectingDebug
    for DSubstitution<Definition, Analysis>
{
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.base.fmt(f, ctx)?;
        write!(f, "(")?;
        let mut first = true;
        if let Err(unresolved) = &self.substitutions {
            for (target, value) in unresolved {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{:?} IS ", target)?;
                value.fmt(f, ctx)?;
            }
        } else if let Ok(resolved) = &self.substitutions {
            for (_target, value) in resolved {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                value.fmt(f, ctx)?;
            }
        }
        write!(f, ")")
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DSubstitution<Definition, Analysis>
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        let mut result = vec![self.base.ptr_clone()];
        match &self.substitutions {
            Ok(resolved) => result.extend(resolved.iter().map(|(_, v)| v.ptr_clone())),
            Err(unresolved) => result.extend(unresolved.iter().map(|(_, v)| v.ptr_clone())),
        }
        result
    }
}

impl<Definition, Analysis> DSubstitution<Definition, Analysis> {
    pub fn new_unresolved(
        base: ItemRef<Definition, Analysis>,
        substitutions: UnresolvedSubstitutions<Definition, Analysis>,
    ) -> Self {
        Self {
            base,
            substitutions: Err(substitutions),
        }
    }

    pub fn new_resolved(
        base: ItemRef<Definition, Analysis>,
        substitutions: Substitutions<Definition, Analysis>,
    ) -> Self {
        Self {
            base,
            substitutions: Ok(substitutions),
        }
    }
}
