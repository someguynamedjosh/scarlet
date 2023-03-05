use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{compound_type::DCompoundType, parameter::ParameterPtr};
use crate::{diagnostic::Diagnostic, environment::{Environment, ItemId}};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Member {
    Unknown,
    IndexIntoUserType(usize),
    Constructor,
}

#[derive(Clone, Debug)]
pub struct DMemberAccess {
    base: ItemId,
    member_name: String,
    member_index: Member,
    r#type: Option<ItemId>,
}

impl DMemberAccess {
    pub fn new(base: ItemId, member_name: String) -> Self {
        Self {
            base,
            member_name,
            member_index: Member::Unknown,
            r#type: None,
        }
    }
}
