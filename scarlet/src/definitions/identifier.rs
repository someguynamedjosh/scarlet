use std::{collections::HashMap, fmt, marker::PhantomData};

use super::parameter::ParameterPtr;
use crate::{
    diagnostic::Diagnostic,
    item::{CddContext, CycleDetectingDebug, ItemDefinition, ItemRef},
};

#[derive(Clone, Debug)]
pub struct DIdentifier<Definition, Analysis> {
    identifier: String,
    _pd: PhantomData<(Definition, Analysis)>,
}

impl<Definition, Analysis> CycleDetectingDebug for DIdentifier<Definition, Analysis> {
    fn fmt(&self, f: &mut fmt::Formatter, _ctx: &mut CddContext) -> fmt::Result {
        write!(f, "IDENTIFIER({})", self.identifier)
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DIdentifier<Definition, Analysis>
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        vec![]
    }
}

impl<Definition, Analysis> DIdentifier<Definition, Analysis> {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            _pd: PhantomData,
        }
    }
}
