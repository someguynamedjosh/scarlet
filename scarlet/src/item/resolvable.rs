pub mod from;
mod identifier;
mod named_member;
mod placeholder;
mod substitution;
mod variable;

use std::{
    convert::Infallible,
    fmt::Debug,
    ops::{FromResidual, Try}, any::Any,
};

pub use identifier::RIdentifier;
pub use named_member::RNamedMember;
pub use placeholder::RPlaceholder;
pub use substitution::RSubstitution;
pub use variable::RVariable;

use crate::{
    environment::Environment,
    item::{ItemDefinition, ItemPtr},
    scope::{LookupInvariantError, Scope}, shared::AnyEq,
};

use super::{check::CheckFeature, dependencies::{DependenciesFeature, Dcc, Dependencies}, equality::EqualityFeature, invariants::InvariantsFeature};

#[derive(Debug)]
pub struct DUnresolved(Box<dyn Resolvable>);

impl DUnresolved {
    pub fn new<R: Resolvable>(resolvable: R) -> Self {
        Self(Box::new(resolvable))
    }
}

impl ItemDefinition for DUnresolved {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(Self::new(self.0.dyn_clone()))
    }
}

impl AnyEq for DUnresolved {
    fn eq(&self, other: &dyn AnyEq) -> bool {
        (other as &dyn Any).downcast_ref::<Self>().map(|other| self.0.eq(other.0));
    }
}

impl CheckFeature for DUnresolved {}
impl DependenciesFeature for DUnresolved {}
impl EqualityFeature for DUnresolved {}
impl InvariantsFeature for DUnresolved {}

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
    Ok(ItemPtr),
    Partial(ItemPtr),
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

    #[allow(unused_variables)]
    fn estimate_dependencies(&self, ctx: &mut Dcc) -> Dependencies {
        Dependencies::new()
    }
}

pub type BoxedResolvable = Box<dyn Resolvable>;
