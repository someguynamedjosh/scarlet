use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{compound_type::DCompoundType, new_value::DNewValue, parameter::ParameterPtr};
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{CddContext, CycleDetectingDebug, ItemDefinition, ItemRef},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Member {
    Unknown,
    IndexIntoUserType(usize),
    Constructor,
}

#[derive(Clone, Debug)]
pub struct DMemberAccess<Definition, Analysis> {
    base: ItemRef<Definition, Analysis>,
    member_name: String,
    member_index: Member,
    r#type: Option<ItemRef<Definition, Analysis>>,
}

impl<Definition, Analysis> CycleDetectingDebug for DMemberAccess<Definition, Analysis> {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.base.fmt(f, ctx)?;
        write!(f, ".{}", self.member_name)
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DMemberAccess<Definition, Analysis>
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        vec![self.base.ptr_clone()]
    }
}

impl<Definition, Analysis> DMemberAccess<Definition, Analysis> {
    pub fn new(base: ItemRef<Definition, Analysis>, member_name: String) -> Self {
        Self {
            base,
            member_name,
            member_index: Member::Unknown,
            r#type: None,
        }
    }
}
