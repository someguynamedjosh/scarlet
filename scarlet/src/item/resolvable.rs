pub mod from;
mod identifier;
mod named_member;
mod placeholder;
mod substitution;
mod variable;

use std::{
    any::Any,
    convert::Infallible,
    fmt::Debug,
    ops::{FromResidual, Try},
};

pub use identifier::RIdentifier;
pub use named_member::RNamedMember;
pub use placeholder::RPlaceholder;
pub use substitution::RSubstitution;
pub use variable::RVariable;

use super::{
    check::CheckFeature,
    dependencies::{Dcc, DepResult, Dependencies, DependenciesFeature, OnlyCalledByDcc},
    equality::EqualityFeature,
    invariants::{Icc, InvariantsFeature, InvariantsResult, OnlyCalledByIcc},
    ContainmentType,
};
use crate::{
    environment::Environment,
    item::{ItemDefinition, ItemPtr},
    scope::{LookupInvariantError, Scope},
    shared::AnyEq,
};

#[derive(Debug)]
pub struct DResolvable(Box<dyn Resolvable>);

impl DResolvable {
    pub fn new<R: Resolvable>(resolvable: R) -> Self {
        Self(Box::new(resolvable))
    }

    pub fn resolvable(&self) -> &dyn Resolvable {
        &*self.0
    }
}

impl ItemDefinition for DResolvable {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(Self(self.0.dyn_clone()))
    }

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)> {
        self.0.contents()
    }
}

impl AnyEq for DResolvable {
    fn eq(&self, other: &dyn AnyEq) -> bool {
        (other as &dyn Any)
            .downcast_ref::<Self>()
            .map(|other| self.0.eq(&*other.0))
            .unwrap_or(false)
    }
}

impl CheckFeature for DResolvable {}
impl DependenciesFeature for DResolvable {
    fn get_dependencies_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        Dependencies::new_error(UnresolvedItemError(this.ptr_clone()))
    }
}
impl EqualityFeature for DResolvable {}
impl InvariantsFeature for DResolvable {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        Err(UnresolvedItemError(this.ptr_clone()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnresolvedItemError(pub ItemPtr);

#[derive(Clone, Debug)]
pub enum ResolveError {
    Unresolved(UnresolvedItemError),
    InvariantDeadEnd(String),
    MaybeInvariantDoesNotExist,
    Placeholder,
}

impl From<UnresolvedItemError> for ResolveError {
    fn from(v: UnresolvedItemError) -> Self {
        Self::Unresolved(v)
    }
}

impl From<LookupInvariantError> for ResolveError {
    fn from(err: LookupInvariantError) -> Self {
        match err {
            LookupInvariantError::Unresolved(err) => Self::Unresolved(err),
            LookupInvariantError::MightNotExist => Self::MaybeInvariantDoesNotExist,
            LookupInvariantError::DefinitelyDoesNotExist => {
                Self::InvariantDeadEnd(format!("No additional info"))
            }
        }
    }
}

pub enum ResolveResult {
    Ok(Box<dyn ItemDefinition>),
    Err(ResolveError),
}

impl FromResidual<Result<Infallible, UnresolvedItemError>> for ResolveResult {
    fn from_residual(residual: Result<Infallible, UnresolvedItemError>) -> Self {
        match residual {
            Ok(ok) => match ok {},
            Err(err) => Self::Err(err.into()),
        }
    }
}

impl FromResidual<Result<Infallible, LookupInvariantError>> for ResolveResult {
    fn from_residual(residual: Result<Infallible, LookupInvariantError>) -> Self {
        match residual {
            Ok(ok) => match ok {},
            Err(err) => Self::Err(err.into()),
        }
    }
}

impl FromResidual<Result<Infallible, ResolveError>> for ResolveResult {
    fn from_residual(residual: Result<Infallible, ResolveError>) -> Self {
        match residual {
            Ok(ok) => match ok {},
            Err(err) => Self::Err(err),
        }
    }
}

pub trait Resolvable: AnyEq + Debug {
    fn is_placeholder(&self) -> bool {
        false
    }
    fn dyn_clone(&self) -> BoxedResolvable;
    fn resolve(
        &self,
        env: &mut Environment,
        this: ItemPtr,
        scope: Box<dyn Scope>,
        limit: u32,
    ) -> ResolveResult;

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)>;

    #[allow(unused_variables)]
    fn estimate_dependencies(&self, ctx: &mut Dcc, affects_return_value: bool) -> Dependencies {
        Dependencies::new()
    }
}

pub type BoxedResolvable = Box<dyn Resolvable>;
