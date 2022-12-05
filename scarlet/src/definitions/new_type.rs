use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
};

use itertools::Itertools;

use super::{builtin::DBuiltin, compound_type::DCompoundType, parameter::ParameterPtr};
use crate::{
    item::{
        parameters::Parameters, CddContext, CycleDetectingDebug, ItemPtr, LazyItemPtr,
        ResolvableItemEnum,
    },
    util::PtrExtension,
};

pub type TypeId = Rc<()>;

#[derive(Clone)]
pub struct DNewType {
    type_id: TypeId,
    fields: Vec<(String, LazyItemPtr<ResolvableItemEnum>)>,
}

impl CycleDetectingDebug for DNewType {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "NEW_TYPE(\n")?;
        for field in &self.fields {
            write!(
                f,
                "   {} IS {}",
                field.0,
                field.1.to_indented_string(ctx, 2)
            )?;
            write!(f, ",\n")?;
        }
        write!(f, ")")
    }
}

impl DNewType {
    pub fn new(fields: Vec<(String, LazyItemPtr<ResolvableItemEnum>)>) -> Self {
        Self {
            fields,
            type_id: TypeId::new(()),
        }
    }

    pub fn is_same_type_as(&self, other: &Self) -> bool {
        self.type_id.is_same_instance_as(&other.type_id)
    }

    pub fn get_fields(&self) -> &[(String, LazyItemPtr<ResolvableItemEnum>)] {
        &self.fields
    }

    pub fn get_type_id(&self) -> TypeId {
        TypeId::clone(&self.type_id)
    }
}
