use std::{collections::HashMap, fmt};

use super::parameter::ParameterPtr;
use crate::{diagnostic::Diagnostic};

#[derive(Clone, Debug)]
pub struct DIdentifier {
    identifier: String,
}

impl DIdentifier {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}
