use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{
    compound_type::{DCompoundType, Type},
    parameter::ParameterPtr,
};
use crate::{
    diagnostic::Diagnostic,
    environment::{Env2, Env3, Environment, ItemId},
};

#[derive(Clone, Debug)]
pub struct DUnresolvedMemberAccess {
    base: ItemId,
    member_name: String,
}

impl DUnresolvedMemberAccess {
    pub fn new(base: ItemId, member_name: String) -> Self {
        Self { base, member_name }
    }

    pub fn base(&self) -> ItemId {
        self.base
    }

    pub fn member_name(&self) -> &str {
        self.member_name.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct DMemberAccess {
    base: ItemId,
    member_index: usize,
}

impl DMemberAccess {
    pub fn new(base: ItemId, base_type: &Type, member_name: &str) -> Result<Self, ()> {
        for (index, (field_name, field_type)) in base_type.get_fields().iter().enumerate() {
            if field_name == member_name {
                return Ok(Self {
                    base,
                    member_index: index,
                });
            }
        }
        Err(())
    }

    pub fn base(&self) -> ItemId {
        self.base
    }

    pub fn member_index(&self) -> usize {
        self.member_index
    }

    pub fn add_type_asserts(&self, env: &mut Env3) {
        // Nothing because we use the type of base to find member_index, so base
        // can never be the wrong type.
    }
}
