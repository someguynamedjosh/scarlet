use std::{
    collections::HashMap,
    fmt::{self},
    rc::Rc,
};

use super::builtin::DBuiltin;
use crate::{
    diagnostic::Position,
    item::{
        parameters::Parameters, CddContext, CycleDetectingDebug, ItemEnum, ItemPtr, LazyItemPtr,
    },
    util::PtrExtension,
};

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Order {
    /// Explicitly defined order, 0-255.
    major_order: u8,
    /// Implicit order by which file it's in.
    file_order: u32,
    /// Implicit order by position in file.
    minor_order: u32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Parameter<I: ItemEnum> {
    order: Order,
    original_type: LazyItemPtr<I>,
}

impl<I: ItemEnum> Parameter<I> {
    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn original_type(&self) -> &LazyItemPtr<I> {
        &self.original_type
    }
}

pub type ParameterPtr<I: ItemEnum> = Rc<Parameter<I>>;

#[derive(Clone)]
pub struct DParameter<I: ItemEnum> {
    parameter: ParameterPtr<I>,
    reduced_type: LazyItemPtr<I>,
}

impl<I: ItemEnum> CycleDetectingDebug for DParameter<I> {
    fn fmt(&self, f: &mut fmt::Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "ANY ")?;
        self.reduced_type.fmt(f, ctx)
    }
}

impl<I: ItemEnum> DParameter<I> {
    pub fn new(major_order: u8, position: Position, r#type: LazyItemPtr<I>) -> Self {
        let order = Order {
            major_order,
            file_order: position.file_index() as _,
            minor_order: position.range().start as _,
        };
        let parameter = Rc::new(Parameter {
            order,
            original_type: r#type.ptr_clone(),
        });
        Self {
            parameter,
            reduced_type: r#type,
        }
    }

    pub fn get_parameter_ptr(&self) -> ParameterPtr<I> {
        Rc::clone(&self.parameter)
    }

    pub fn get_parameter(&self) -> &Parameter<I> {
        &*self.parameter
    }

    pub fn get_type(&self) -> &LazyItemPtr<I> {
        &self.reduced_type
    }
}
