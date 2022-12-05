use std::{collections::HashMap, fmt};

use super::parameter::ParameterPtr;
use crate::{
    diagnostic::Diagnostic,
    item::{parameters::Parameters, CddContext, CycleDetectingDebug, ItemPtr, LazyItemPtr},
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

impl DIdentifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}
