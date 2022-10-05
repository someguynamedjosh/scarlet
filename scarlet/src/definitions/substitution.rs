use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;

use super::{builtin::DBuiltin, parameter::ParameterPtr};
use crate::{
    item::{
        query::{
            no_type_check_errors, ChildrenQuery, ParametersQuery, Query, QueryContext,
            TypeCheckQuery, TypeQuery,
        },
        type_hints::TypeHint,
        CddContext, CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
    },
    shared::OrderedMap,
    util::PtrExtension,
};

pub type Substitutions = OrderedMap<ParameterPtr, ItemPtr>;

#[derive(Clone)]
pub struct DSubstitution {
    base: ItemPtr,
    substitutions: Substitutions,
}

impl CycleDetectingDebug for DSubstitution {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.base.fmt(f, ctx)?;
        write!(f, "(")?;
        let mut first = true;
        for (target, value) in &self.substitutions {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{:?} IS ", target)?;
            value.fmt(f, ctx)?;
        }
        write!(f, ")")
    }
}

impl ItemDefinition for DSubstitution {
    fn children(&self) -> Vec<ItemPtr> {
        std::iter::once(self.base.ptr_clone())
            .chain(self.substitutions.iter().map(|(_, v)| v.ptr_clone()))
            .collect()
    }

    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        todo!()
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        todo!()
    }

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        let mut carried_args = args.clone();
        let mut new_args = HashMap::new();
        for (target, value) in &self.substitutions {
            new_args.insert(target.ptr_clone(), value.reduce(args));
            carried_args.remove(target);
        }
        self.base.reduce(&new_args)
    }
}

impl DSubstitution {
    pub fn new(base: ItemPtr, substitutions: Substitutions) -> Self {
        Self {
            base,
            substitutions,
        }
    }
}
