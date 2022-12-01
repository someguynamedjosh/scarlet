use std::{
    cell::Ref,
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
};

use itertools::Itertools;
use owning_ref::OwningRef;

use super::{compound_type::DCompoundType, parameter::ParameterPtr};
use crate::{
    diagnostic::Diagnostic,
    environment::{r#true, Environment},
    item::{
        parameters::Parameters,
        query::{ParametersQuery, Query, QueryContext, ResolveQuery, TypeCheckQuery, TypeQuery},
        CddContext, CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, Debug)]
pub enum DPlaceholder {
    Resolved(ItemPtr),
    Reduced(ItemPtr, HashMap<ParameterPtr, ItemPtr>),
}

impl CycleDetectingDebug for DPlaceholder {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        match self {
            Self::Resolved(base) => {
                write!(f, "PLACEHOLDER_RESOLVED(")?;
                base.fmt(f, ctx)?;
                write!(f, ")")
            }
            Self::Reduced(base, _) => {
                write!(f, "PLACEHOLDER_REDUCED(")?;
                base.fmt(f, ctx)?;
                write!(f, ")")
            }
        }
    }
}

impl ItemDefinition for DPlaceholder {
    fn children(&self) -> Vec<ItemPtr> {
        match self {
            DPlaceholder::Resolved(base) | DPlaceholder::Reduced(base, _) => {
                vec![base.ptr_clone()]
            }
        }
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        todo!()
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        todo!()
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        todo!()
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        todo!()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        todo!()
    }

    fn without_placeholders(&self, this: &ItemPtr) -> ItemPtr {
        match self {
            Self::Resolved(base) => base
                .query_resolved(&mut Environment::root_query())
                .unwrap()
                .without_placeholders(),
            Self::Reduced(base, args) => base.reduce(&args),
        }
    }
}
