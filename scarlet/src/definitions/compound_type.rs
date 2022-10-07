use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;
use maplit::hashmap;

use super::{builtin::DBuiltin, new_type::DNewType, parameter::ParameterPtr};
use crate::item::{
    query::{
        no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
    },
    CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
};

#[derive(Clone)]
pub struct DCompoundType {
    // These are ORed together. ANDing them would result in an empty type any
    // time you have at least 2 non-identical components.
    component_types: HashMap<usize, ItemPtr>,
}

impl CycleDetectingDebug for DCompoundType {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "UNION(\n")?;
        for (_id, r#type) in &self.component_types {
            write!(f, "   {}", r#type.to_indented_string(ctx, 1))?;
            write!(f, ",\n")?;
        }
        write!(f, ")")
    }
}

impl ItemDefinition for DCompoundType {
    fn children(&self) -> Vec<ItemPtr> {
        self.component_types
            .iter()
            .map(|t| t.1.ptr_clone())
            .collect_vec()
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        _ctx: &mut QueryContext<ParametersQuery>,
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
        this.ptr_clone()
    }
}

impl DCompoundType {
    pub fn new(base_type: ItemPtr, base_type_id: usize) -> Self {
        let component_types = hashmap![base_type_id => base_type];
        Self { component_types }
    }

    pub fn get_component_types(&self) -> &HashMap<usize, ItemPtr> {
        &self.component_types
    }

    pub fn constructor(&self, this: &ItemPtr) -> Option<ItemPtr> {
        if self.component_types.len() == 1 {
            let r#type = self.component_types.iter().next().unwrap().1.ptr_clone();
            // "Type" can also be a component type which doesn't have a constructor.
            let def = r#type.downcast_definition::<DNewType>()?;
            Some(def.constructor(this))
        } else {
            None
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut component_types = self.component_types.clone();
        component_types.extend(
            other
                .component_types
                .iter()
                .map(|(id, ty)| (*id, ty.ptr_clone())),
        );
        Self { component_types }
    }

    pub fn is_exactly_type(&self) -> bool {
        self.component_types.len() == 1 && self.component_types.contains_key(&0)
    }

    /// False is non-definitive here.
    pub fn is_subtype_of(&self, other: &Self) -> bool {
        for (key, _) in &self.component_types {
            if !other.component_types.contains_key(key) {
                return false;
            }
        }
        true
    }
}
