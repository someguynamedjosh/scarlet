use std::{
    collections::HashMap,
    fmt::{self},
    rc::Rc,
};

use super::builtin::DBuiltin;
use crate::{diagnostic::Position, environment::ItemId, util::PtrExtension};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Order {
    /// Explicitly defined order, 0-255.
    major_order: u8,
    /// Implicit order by which file it's in.
    file_order: u32,
    /// Implicit order by position in file.
    minor_order: u32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Parameter {
    order: Order,
    original_type: ItemId,
}

impl Parameter {
    pub fn order(&self) -> Order {
        self.order
    }

    pub fn original_type(&self) -> ItemId {
        self.original_type
    }
}

pub type ParameterPtr = Rc<Parameter>;

#[derive(Clone, Debug)]
pub struct DParameter {
    parameter: ParameterPtr,
    reduced_type: ItemId,
}

impl DParameter {
    pub fn new(major_order: u8, position: Position, r#type: ItemId) -> Self {
        let order = Order {
            major_order,
            file_order: position.file_index() as _,
            minor_order: position.range().start as _,
        };
        let parameter = Rc::new(Parameter {
            order,
            original_type: r#type,
        });
        Self {
            parameter,
            reduced_type: r#type,
        }
    }

    pub fn get_parameter_ptr(&self) -> ParameterPtr {
        Rc::clone(&self.parameter)
    }

    pub fn get_parameter(&self) -> &Parameter {
        &*self.parameter
    }

    pub fn get_type(&self) -> &ItemId {
        &self.reduced_type
    }
}
