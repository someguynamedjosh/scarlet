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

pub type Substitutions<Definition, Analysis> = OrderedMap<
    (
        ItemRef<Definition, Analysis>,
        ParameterPtr<Definition, Analysis>,
    ),
    ItemRef<Definition, Analysis>,
>;

pub struct DSubstitution<Definition, Analysis> {
    base: ItemRef<Definition, Analysis>,
    substitutions: Substitutions<Definition, Analysis>,
}

impl<Definition, Analysis> Clone for DSubstitution<Definition, Analysis> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.ptr_clone(),
            substitutions: self.substitutions.clone(),
        }
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> CycleDetectingDebug
    for DSubstitution<Definition, Analysis>
{
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.base.fmt(f, ctx)?;
        write!(f, "(")?;
        let mut first = true;
        for (_target, value) in &self.substitutions {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            value.fmt(f, ctx)?;
        }
        write!(f, ")")
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DSubstitution<Definition, Analysis>
{
    fn map_children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        todo!()
    }
}

impl<Definition, Analysis> DSubstitution<Definition, Analysis> {
    pub fn new(
        base: ItemRef<Definition, Analysis>,
        substitutions: Substitutions<Definition, Analysis>,
    ) -> Self {
        Self {
            base,
            substitutions,
        }
    }
}
