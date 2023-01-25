use std::{collections::HashMap, fmt, marker::PhantomData};

use super::parameter::ParameterPtr;
use crate::{
    diagnostic::Diagnostic,
    item::{CddContext, CycleDetectingDebug, ItemDefinition, ItemRef},
};

pub struct DIdentifier<Definition, Analysis> {
    identifier: String,
    _pd: PhantomData<(Definition, Analysis)>,
}

impl<Definition, Analysis> Clone for DIdentifier<Definition, Analysis> {
    fn clone(&self) -> Self {
        Self {
            identifier: self.identifier.clone(),
            _pd: self._pd.clone(),
        }
    }
}

impl<Definition, Analysis> CycleDetectingDebug for DIdentifier<Definition, Analysis> {
    fn fmt(&self, f: &mut fmt::Formatter, _ctx: &mut CddContext) -> fmt::Result {
        write!(f, "IDENTIFIER({})", self.identifier)
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DIdentifier<Definition, Analysis>
{
    fn map_children(&self) -> Vec<ItemRef<Definition, Analysis>> {
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

    pub(crate) fn identifier(&self) -> &str {
        &self.identifier
    }
}
