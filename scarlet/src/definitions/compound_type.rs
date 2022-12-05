use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;
use maplit::hashmap;

use super::{builtin::DBuiltin, new_type::DNewType, parameter::ParameterPtr};
use crate::item::{
    parameters::Parameters,
    query::{
        no_type_check_errors, ParametersQuery, Query, QueryContext, ResolveQuery, TypeCheckQuery,
        TypeQuery,
    },
    CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr, LazyItemPtr,
};

#[derive(Clone, Debug)]
pub struct DCompoundType {
    // These are ORed together. ANDing them would result in an empty type any
    // time you have at least 2 non-identical components.
    component_types: HashMap<usize, LazyItemPtr>,
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

impl ItemDefinition for DCompoundType {
    fn children(&self) -> Vec<LazyItemPtr> {
        self.component_types
            .iter()
            .map(|t| t.1.ptr_clone())
            .collect_vec()
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(LazyItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        let mut result = Parameters::new_empty();
        for typ in &self.component_types {
            result.append(typ.1.evaluate().unwrap().query_parameters(ctx));
        }
        result
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(DBuiltin::r#type().into_ptr().into_lazy())
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
        Ok(this.ptr_clone())
    }

    fn reduce(&self, this: &ItemPtr, _args: &HashMap<ParameterPtr, LazyItemPtr>) -> ItemPtr {
        this.ptr_clone()
    }
}

impl DCompoundType {
    pub fn new(base_type: LazyItemPtr, base_type_id: usize) -> Self {
        let component_types = hashmap![base_type_id => base_type];
        Self { component_types }
    }

    pub fn get_component_types(&self) -> &HashMap<usize, LazyItemPtr> {
        &self.component_types
    }

    pub fn constructor(&self, this: &ItemPtr) -> Option<ItemPtr> {
        if self.component_types.len() == 1 {
            let r#type = self
                .component_types
                .iter()
                .next()
                .unwrap()
                .1
                .ptr_clone()
                .evaluate()
                .unwrap();
            // "Type" can also be a component type which doesn't have a constructor.
            let def = r#type.downcast_definition::<DNewType>()?;
            Some(def.constructor(&r#type, this))
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
