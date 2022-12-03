use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
};

use itertools::Itertools;

use super::{
    builtin::DBuiltin, compound_type::DCompoundType, new_value::DNewValue, parameter::ParameterPtr,
};
use crate::{
    item::{
        parameters::Parameters,
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, ResolveQuery,
            TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
    },
    util::PtrExtension,
};

pub type TypeId = Rc<()>;

#[derive(Clone)]
pub struct DNewType {
    type_id: TypeId,
    fields: Vec<(String, ItemPtr)>,
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

impl ItemDefinition for DNewType {
    fn children(&self) -> Vec<ItemPtr> {
        self.fields.iter().map(|(_, f)| f.ptr_clone()).collect_vec()
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        let mut result = Parameters::new_empty();
        for (_, field) in &self.fields {
            result.append(field.query_type(ctx).unwrap().query_parameters(ctx));
        }
        result
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(DBuiltin::r#type().into_ptr())
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        let this_type = Self {
            fields: self
                .fields
                .iter()
                .map(|(name, value)| value.query_resolved(ctx).map(|value| (name.clone(), value)))
                .try_collect()?,
            type_id: self.type_id.ptr_clone(),
        }
        .into_ptr_mimicking(this);
        Ok(
            DCompoundType::new(this_type, TypeId::as_ptr(&self.type_id).to_bits())
                .into_ptr_mimicking(this),
        )
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        this.ptr_clone()
    }
}

impl DNewType {
    pub fn new(fields: Vec<(String, ItemPtr)>) -> Self {
        Self {
            fields,
            type_id: TypeId::new(()),
        }
    }

    pub fn is_same_type_as(&self, other: &Self) -> bool {
        self.type_id.is_same_instance_as(&other.type_id)
    }

    pub fn get_fields(&self) -> &[(String, ItemPtr)] {
        &self.fields
    }

    pub fn get_type_id(&self) -> TypeId {
        TypeId::clone(&self.type_id)
    }

    pub fn constructor(&self, this: &ItemPtr, mimicking: &ItemPtr) -> ItemPtr {
        DNewValue::new(
            this.ptr_clone(),
            self.fields.iter().map(|f| f.1.ptr_clone()).collect(),
        )
        .into_ptr_mimicking(mimicking)
    }
}
