use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{
    builtin::DBuiltin,
    parameter::{DParameter, ParameterPtr},
};
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
    },
    shared::OrderedMap,
    util::PtrExtension,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnresolvedTarget {
    Positional,
    Named(String),
}

pub type UnresolvedSubstitutions = OrderedMap<UnresolvedTarget, ItemPtr>;
pub type Substitutions = OrderedMap<(ItemPtr, ParameterPtr), ItemPtr>;

#[derive(Clone)]
pub struct DSubstitution {
    base: ItemPtr,
    substitutions: Result<Substitutions, UnresolvedSubstitutions>,
}

impl CycleDetectingDebug for DSubstitution {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.base.fmt(f, ctx)?;
        write!(f, "(")?;
        let mut first = true;
        if let Err(unresolved) = &self.substitutions {
            for (target, value) in unresolved {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{:?} IS ", target)?;
                value.fmt(f, ctx)?;
            }
        } else if let Ok(resolved) = &self.substitutions {
            for (target, value) in resolved {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{:?} IS ", target)?;
                value.fmt(f, ctx)?;
            }
        }
        write!(f, ")")
    }
}

impl ItemDefinition for DSubstitution {
    fn children(&self) -> Vec<ItemPtr> {
        let mut result = vec![self.base.ptr_clone()];
        match &self.substitutions {
            Ok(resolved) => result.extend(resolved.iter().map(|(_, v)| v.ptr_clone())),
            Err(unresolved) => result.extend(unresolved.iter().map(|(_, v)| v.ptr_clone())),
        }
        result
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        let subs = self.substitutions.as_ref().unwrap();
        subs.iter()
            .map(|(target, value)| {
                (
                    value.ptr_clone(),
                    DBuiltin::is_subtype_of(
                        value.query_type(&mut Environment::root_query()).unwrap(),
                        target.0.ptr_clone(),
                    )
                    .into_ptr(),
                )
            })
            .collect()
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        let mut result = self.base.query_parameters(ctx);
        if self.substitutions.is_err() {
            result.mark_excluding(this.ptr_clone());
            return result;
        }
        let mut new_args = HashMap::new();
        for (target, value) in self.substitutions.as_ref().unwrap() {
            new_args.insert(target.1.clone(), value.reduce(&HashMap::new()));
        }
        result.reduce_type(&new_args);
        result
    }

    fn recompute_type(&self, ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        let mut new_args = HashMap::new();
        for (target, value) in self.substitutions.as_ref().unwrap() {
            new_args.insert(target.1.clone(), value.reduce(&HashMap::new()));
        }
        Some(self.base.query_type(ctx)?.reduce(&new_args))
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, _this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        let mut carried_args = args.clone();
        let mut new_args = HashMap::new();
        for (target, value) in self.substitutions.as_ref().unwrap() {
            new_args.insert(target.1.clone(), value.reduce(args));
            carried_args.remove(&target.1);
        }
        new_args.extend(carried_args.into_iter());
        self.base.reduce(&new_args)
    }

    fn resolve(&mut self, this: &ItemPtr) -> Result<(), Diagnostic> {
        let mut params = self.base.query_parameters(&mut Environment::root_query());
        if let Err(unresolved) = &self.substitutions {
            if params.excludes_any_parameters() {
                return Err(Diagnostic::new()
                    .with_text_error(format!("Cannot determine parameters of base."))
                    .with_item_error(&self.base));
            }
            let mut resolved = OrderedMap::new();
            for (target, value) in unresolved {
                match target {
                    UnresolvedTarget::Positional => todo!("Positional arguments"),
                    UnresolvedTarget::Named(name) => {
                        let gen_error = || {
                            Diagnostic::new().with_text_error(format!(
                                "No parameter named \"{}\" in the scope of the base.",
                                name
                            ))
                        };
                        let param = self.base.lookup_identifier(name).ok_or_else(gen_error)?;
                        let param = param
                            .downcast_definition::<DParameter>()
                            .ok_or_else(gen_error)?;
                        let param = params.remove(param.get_parameter()).ok_or_else(gen_error)?;
                        resolved.insert(param, value.ptr_clone())
                    }
                }
            }
            self.substitutions = Ok(resolved);
        }
        Ok(())
    }
}

impl DSubstitution {
    pub fn new_unresolved(base: ItemPtr, substitutions: UnresolvedSubstitutions) -> Self {
        Self {
            base,
            substitutions: Err(substitutions),
        }
    }

    pub fn new_resolved(base: ItemPtr, substitutions: Substitutions) -> Self {
        Self {
            base,
            substitutions: Ok(substitutions),
        }
    }
}
