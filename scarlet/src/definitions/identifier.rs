use std::{collections::HashMap, fmt};

use super::{parameter::ParameterPtr, reference::DReference};
use crate::{
    diagnostic::Diagnostic,
    item::{CddContext, CycleDetectingDebug, ItemDefinition, ItemRef},
};

#[derive(Clone, Debug)]
pub struct DIdentifier {
    identifier: String,
}

impl CycleDetectingDebug for DIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter, _ctx: &mut CddContext) -> fmt::Result {
        write!(f, "IDENTIFIER({})", self.identifier)
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DIdentifier
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        vec![]
    }
}

impl DIdentifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}
