use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;
use maplit::hashmap;

use super::{builtin::DBuiltin, new_type::DNewType, parameter::ParameterPtr};
use crate::item::{
    parameters::Parameters, CddContext, CycleDetectingDebug, ItemPtr, LazyItemPtr, ResolvedItemEnum,
};

#[derive(Clone, Debug)]
pub struct DCompoundType {
    // These are ORed together. ANDing them would result in an empty type any
    // time you have at least 2 non-identical components.
    component_types: HashMap<usize, LazyItemPtr<ResolvedItemEnum>>,
}

impl CycleDetectingDebug for DCompoundType {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        if self.component_types.len() == 1 {
            self.component_types.iter().next().unwrap().1.fmt(f, ctx)
        } else {
            write!(f, "UNION(\n")?;
            for (_id, r#type) in &self.component_types {
                write!(f, "   {}", r#type.to_indented_string(ctx, 1))?;
                write!(f, ",\n")?;
            }
            write!(f, ")")
        }
    }
}

impl DCompoundType {
    pub fn new(base_type: LazyItemPtr<ResolvedItemEnum>, base_type_id: usize) -> Self {
        let component_types = hashmap![base_type_id => base_type];
        Self { component_types }
    }

    pub fn get_component_types(&self) -> &HashMap<usize, LazyItemPtr<ResolvedItemEnum>> {
        &self.component_types
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
