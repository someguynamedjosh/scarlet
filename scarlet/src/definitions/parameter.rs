use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
};

use super::builtin::DBuiltin;
use crate::{
    diagnostic::Position,
    environment::Environment,
    item::{
        query::{ChildrenQuery, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery},
        type_hints::TypeHint,
        CddContext, CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
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
    order: Order,
}

impl Parameter {
    pub fn order(&self) -> &Order {
        &self.order
    }
}

pub type ParameterPtr = Rc<Parameter>;

#[derive(Clone)]
pub struct DParameter {
    parameter: ParameterPtr,
    r#type: ItemPtr,
}

impl CycleDetectingDebug for DParameter {
    fn fmt(&self, f: &mut fmt::Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "ANY ")?;
        self.r#type.fmt(f, ctx)
    }
}

impl ItemDefinition for DParameter {
    fn children(&self) -> Vec<ItemPtr> {
        vec![self.r#type.ptr_clone()]
    }

    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![
            (
                this.ptr_clone(),
                DBuiltin::is_subtype_of(this.ptr_clone(), self.r#type.ptr_clone()).into_ptr(),
            ),
            (
                this.ptr_clone(),
                DBuiltin::is_type(self.r#type.ptr_clone()).into_ptr(),
            ),
        ]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(self.r#type.ptr_clone())
    }

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        todo!()
    }

    fn reduce(
        &self,
        this: &ItemPtr,
        args: &HashMap<ParameterPtr, ItemPtr>,
        env: &Environment,
    ) -> ItemPtr {
        if let Some(value) = args.get(&self.parameter) {
            value.ptr_clone()
        } else {
            let r#type = self.r#type.reduce(args, env);
            if r#type.is_same_instance_as(&self.r#type) {
                this.ptr_clone()
            } else {
                DParameter {
                    parameter: Rc::clone(&self.parameter),
                    r#type,
                }
                .into_ptr()
            }
        }
    }
}

impl DParameter {
    pub fn new(major_order: u8, position: Position, r#type: ItemPtr) -> Self {
        let order = Order {
            major_order,
            file_order: position.file_index() as _,
            minor_order: position.range().start as _,
        };
        let parameter = Rc::new(Parameter { order });
        Self { parameter, r#type }
    }

    pub fn get_parameter(&self) -> &Parameter {
        &*self.parameter
    }

    pub fn get_type(&self) -> &ItemPtr {
        &self.r#type
    }
}
