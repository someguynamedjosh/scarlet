use std::{collections::HashMap, fmt};

use super::parameter::ParameterPtr;
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{
        parameters::Parameters,
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, Debug)]
enum Transformation {
    None,
    Resolve,
}

#[derive(Clone, Debug)]
pub struct DReference {
    base: ItemPtr,
    transformation: Transformation,
    indirect: bool,
}

impl CycleDetectingDebug for DReference {
    fn fmt(&self, f: &mut fmt::Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "REFERENCE(")?;
        match self.target() {
            Ok(i) => i.fmt(f, ctx)?,
            Err(_) => write!(f, "ERROR")?,
        }
        write!(f, ")")
    }
}

impl ItemDefinition for DReference {
    fn children(&self) -> Vec<ItemPtr> {
        if self.indirect {
            vec![]
        } else {
            vec![self.base.ptr_clone()]
        }
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        if self.indirect {
            vec![]
        } else {
            self.base.collect_constraints()
        }
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        self.base.query_parameters(ctx)
    }

    fn recompute_type(&self, ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        self.base.query_type(ctx)
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        self.base.reduced(args, true)
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<crate::item::query::ResolveQuery>,
    ) -> <crate::item::query::ResolveQuery as Query>::Result {
        self.base.resolve_now(ctx)
    }
}

impl DReference {
    pub fn new(target: ItemPtr) -> Self {
        Self {
            base: target,
            transformation: Transformation::None,
            indirect: false,
        }
    }

    pub fn new_indirect(target: ItemPtr) -> Self {
        Self {
            base: target,
            transformation: Transformation::None,
            indirect: true,
        }
    }

    pub fn new_resolve(base: ItemPtr) -> Self {
        Self {
            base,
            transformation: Transformation::Resolve,
            indirect: false,
        }
    }

    pub fn new_indirect_resolve(base: ItemPtr) -> Self {
        Self {
            base,
            transformation: Transformation::Resolve,
            indirect: true,
        }
    }

    pub fn target(&self) -> Result<ItemPtr, Diagnostic> {
        match self.transformation {
            Transformation::None => Ok(self.base.ptr_clone()),
            Transformation::Resolve => self.base.resolve_now(&mut Environment::root_query()),
        }
    }
}
