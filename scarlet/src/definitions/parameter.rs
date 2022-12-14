use std::{
    collections::HashMap,
    fmt::{self},
    hash::Hash,
    rc::Rc,
};

use super::builtin::DBuiltin;
use crate::{
    diagnostic::Position,
    item::{CddContext, CycleDetectingDebug, ItemDefinition, ItemRef},
    util::PtrExtension,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Order {
    /// Explicitly defined order, 0-255.
    major_order: u8,
    /// Implicit order by which file it's in.
    file_order: u32,
    /// Implicit order by position in file.
    minor_order: u32,
}

pub struct Parameter<Definition, Analysis> {
    order: Order,
    original_type: ItemRef<Definition, Analysis>,
}

impl<Definition, Analysis> Clone for Parameter<Definition, Analysis> {
    fn clone(&self) -> Self {
        Self {
            order: self.order,
            original_type: self.original_type.ptr_clone(),
        }
    }
}

impl<Definition, Analysis> PartialEq for Parameter<Definition, Analysis> {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order && self.original_type == other.original_type
    }
}

impl<Definition, Analysis> Eq for Parameter<Definition, Analysis> {}

impl<Definition, Analysis> Hash for Parameter<Definition, Analysis> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.order.hash(state);
        self.original_type.hash(state);
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> std::fmt::Debug
    for Parameter<Definition, Analysis>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parameter")
            .field("order", &self.order)
            .field("original_type", &self.original_type)
            .finish()
    }
}

impl<Definition, Analysis> Parameter<Definition, Analysis> {
    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn original_type(&self) -> &ItemRef<Definition, Analysis> {
        &self.original_type
    }
}

pub type ParameterPtr<Definition, Analysis> = Rc<Parameter<Definition, Analysis>>;

pub struct DParameter<Definition, Analysis> {
    parameter: ParameterPtr<Definition, Analysis>,
    reduced_type: ItemRef<Definition, Analysis>,
}

impl<Definition, Analysis> Clone for DParameter<Definition, Analysis> {
    fn clone(&self) -> Self {
        Self {
            parameter: self.parameter.ptr_clone(),
            reduced_type: self.reduced_type.ptr_clone(),
        }
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> CycleDetectingDebug
    for DParameter<Definition, Analysis>
{
    fn fmt(&self, f: &mut fmt::Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "ANY ")?;
        self.reduced_type.fmt(f, ctx)
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DParameter<Definition, Analysis>
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        vec![self.reduced_type.ptr_clone()]
    }
}

impl<Definition, Analysis> DParameter<Definition, Analysis> {
    pub fn new(major_order: u8, position: Position, r#type: ItemRef<Definition, Analysis>) -> Self {
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

    pub fn get_parameter_ptr(&self) -> ParameterPtr<Definition, Analysis> {
        Rc::clone(&self.parameter)
    }

    pub fn get_parameter(&self) -> &Parameter<Definition, Analysis> {
        &*self.parameter
    }

    pub fn get_type(&self) -> &ItemRef<Definition, Analysis> {
        &self.reduced_type
    }
}
