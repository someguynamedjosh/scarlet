use std::{
    collections::HashMap,
    fmt::{self},
    rc::Rc,
};

use super::builtin::DBuiltin;
use crate::{
    diagnostic::Position,
    item::{
        parameters::Parameters,
        query::{ParametersQuery, Query, QueryContext, ResolveQuery, TypeCheckQuery, TypeQuery},
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
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
pub struct Parameter {
    order: Order,
    original_type: ItemPtr,
}

impl Parameter {
    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn original_type(&self) -> &ItemPtr {
        &self.original_type
    }
}

pub type ParameterPtr = Rc<Parameter>;

#[derive(Clone)]
pub struct DParameter {
    parameter: ParameterPtr,
    reduced_type: ItemPtr,
}

impl CycleDetectingDebug for DParameter {
    fn fmt(&self, f: &mut fmt::Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "ANY ")?;
        self.reduced_type.fmt(f, ctx)
    }
}

impl ItemDefinition for DParameter {
    fn children(&self) -> Vec<ItemPtr> {
        vec![self.reduced_type.ptr_clone()]
    }

    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![(
            this.ptr_clone(),
            DBuiltin::is_type(self.reduced_type.ptr_clone()).into_ptr(),
        )]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        let mut result = self.reduced_type.query_parameters(ctx);
        result.insert(self.reduced_type.ptr_clone(), self.parameter.ptr_clone());
        result
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(self.reduced_type.ptr_clone())
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        todo!()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        if let Some(value) = args.get(&self.parameter) {
            value.ptr_clone()
        } else {
            let r#type = self.reduced_type.reduce(args);
            if r#type.is_same_instance_as(&self.reduced_type) {
                this.ptr_clone()
            } else {
                Self {
                    parameter: Rc::clone(&self.parameter),
                    reduced_type: r#type,
                }
                .into_ptr_mimicking(this)
            }
        }
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        let r#type = self.reduced_type.query_resolved(ctx)?;
        if r#type.is_same_instance_as(&self.reduced_type) {
            Ok(this.ptr_clone())
        } else {
            Ok(Self {
                parameter: Rc::clone(&self.parameter),
                reduced_type: r#type,
            }
            .into_ptr_mimicking(this))
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
        let parameter = Rc::new(Parameter {
            order,
            original_type: r#type.ptr_clone(),
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

    pub fn get_type(&self) -> &ItemPtr {
        &self.reduced_type
    }
}
