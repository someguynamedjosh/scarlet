mod identifier;
mod named_member;
mod placeholder;
mod substitution;
mod variable;

use std::fmt::Debug;

pub use identifier::RIdentifier;
pub use named_member::RNamedMember;
pub use placeholder::RPlaceholder;
pub use substitution::RSubstitution;
pub use variable::RVariable;

use crate::{
    constructs::ConstructDefinition,
    environment::{Environment, UnresolvedConstructError},
    scope::Scope,
};

#[derive(Clone, Debug)]
pub enum ResolveError {
    UnresolvedConstruct(UnresolvedConstructError),
    InsufficientInvariants(String),
}

impl From<UnresolvedConstructError> for ResolveError {
    fn from(v: UnresolvedConstructError) -> Self {
        Self::UnresolvedConstruct(v)
    }
}

pub type ResolveResult<'x> = Result<ConstructDefinition<'x>, ResolveError>;

pub trait Resolvable<'x>: Debug {
    fn is_placeholder(&self) -> bool {
        false
    }
    fn dyn_clone(&self) -> BoxedResolvable<'x>;
    fn resolve(
        &self,
        env: &mut Environment<'x>,
        scope: Box<dyn Scope>,
        limit: u32,
    ) -> ResolveResult<'x>;
}

pub type BoxedResolvable<'x> = Box<dyn Resolvable<'x> + 'x>;
