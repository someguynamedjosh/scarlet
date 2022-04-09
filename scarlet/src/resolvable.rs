pub mod from;
mod identifier;
mod named_member;
mod placeholder;
mod substitution;
mod variable;

use std::{
    convert::Infallible,
    fmt::Debug,
    ops::{FromResidual, Try},
};

pub use identifier::RIdentifier;
pub use named_member::RNamedMember;
pub use placeholder::RPlaceholder;
pub use substitution::RSubstitution;
pub use variable::RVariable;

use crate::{
    constructs::{ItemDefinition, ItemId},
    environment::{dependencies::Dependencies, Environment, UnresolvedItemError},
    scope::{LookupInvariantError, Scope},
};

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
    Ok(ItemDefinition),
    Partial(ItemDefinition),
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

pub trait Resolvable<'x>: Debug {
    fn is_placeholder(&self) -> bool {
        false
    }
    fn dyn_clone(&self) -> BoxedResolvable<'x>;
    fn resolve(
        &self,
        env: &mut Environment<'x>,
        this: ItemId,
        scope: Box<dyn Scope>,
        limit: u32,
    ) -> ResolveResult;
    #[allow(unused_variables)]
    fn estimate_dependencies(&self, env: &mut Environment) -> Dependencies {
        Dependencies::new()
    }
}

pub type BoxedResolvable<'x> = Box<dyn Resolvable<'x> + 'x>;
