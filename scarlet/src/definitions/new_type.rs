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
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
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
        _ctx: &mut QueryContext<ParametersQuery>,
       this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
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

    fn reduce(&self, this: &ItemPtr, _args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        DCompoundType::new(this.ptr_clone(), TypeId::as_ptr(&self.type_id).to_bits()).into_ptr()
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

    pub fn constructor(&self, this: &ItemPtr) -> ItemPtr {
        DNewValue::new(
            this.ptr_clone(),
            self.fields.iter().map(|f| f.1.ptr_clone()).collect(),
        )
        .into_ptr()
    }
}
