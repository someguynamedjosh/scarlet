use std::{
    fmt::{self, Formatter},
    rc::Rc,
};

use crate::{
    diagnostic::Position,
    item::{
        query::{ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery},
        CycleDetectingDebug, Item, ItemDefinition, ItemPtr,
    },
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
pub struct Parameter {
    r#type: ItemPtr,
    order: Order,
}

impl Parameter {
    pub fn r#type(&self) -> &ItemPtr {
        &self.r#type
    }

    pub fn order(&self) -> &Order {
        &self.order
    }
}

impl CycleDetectingDebug for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>, stack: &[*const Item]) -> fmt::Result {
        write!(f, "ANY ")?;
        self.r#type.fmt(f, stack)
    }
}

pub type ParameterPtr = Rc<Parameter>;

#[derive(Clone)]
pub struct DParameter(ParameterPtr);

impl CycleDetectingDebug for DParameter {
    fn fmt(&self, f: &mut fmt::Formatter, stack: &[*const Item]) -> fmt::Result {
        self.0.fmt(f, stack)
    }
}

impl ItemDefinition for DParameter {
    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(self.0.r#type.ptr_clone())
    }

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        todo!()
    }
}

impl DParameter {
    pub fn new(major_order: u8, position: Position, r#type: ItemPtr) -> Self {
        let order = Order {
            major_order,
            file_order: position.file_index() as _,
            minor_order: position.range().start as _,
        };
        let parameter = Rc::new(Parameter { order, r#type });
        Self(parameter)
    }
}
