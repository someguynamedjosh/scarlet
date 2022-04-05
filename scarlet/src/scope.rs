use std::fmt::Debug;

use maplit::hashset;

use crate::{
    constructs::ItemId,
    environment::{
        discover_equality::Equal, invariants::Invariant, Environment, UnresolvedItemError,
    },
};

pub type LookupIdentResult = Result<Option<ItemId>, UnresolvedItemError>;
pub type ReverseLookupIdentResult = Result<Option<String>, UnresolvedItemError>;
pub type LookupInvariantResult = Result<Invariant, LookupInvariantError>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LookupInvariantError {
    Unresolved(UnresolvedItemError),
    MightNotExist,
    DefinitelyDoesNotExist,
}

impl From<UnresolvedItemError> for LookupInvariantError {
    fn from(v: UnresolvedItemError) -> Self {
        Self::Unresolved(v)
    }
}

pub trait Scope: Debug {
    fn dyn_clone(&self) -> Box<dyn Scope>;

    fn is_placeholder(&self) -> bool {
        false
    }

    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> LookupIdentResult;
    fn local_reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ItemId,
    ) -> ReverseLookupIdentResult;
    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ItemId,
        limit: u32,
    ) -> LookupInvariantResult;
    fn parent(&self) -> Option<ItemId>;

    fn lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> LookupIdentResult {
        if let Some(result) = self.local_lookup_ident(env, ident)? {
            Ok(Some(result))
        } else if let Some(parent) = self.parent() {
            env.get_item(parent)
                .scope
                .dyn_clone()
                .lookup_ident(env, ident)
        } else {
            Ok(None)
        }
    }

    fn reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ItemId,
    ) -> ReverseLookupIdentResult {
        if let Some(result) = self.local_reverse_lookup_ident(env, value)? {
            if result.len() > 0 {
                return Ok(Some(result.to_owned()));
            }
        }
        if let Some(parent) = self.parent() {
            env.get_item(parent)
                .scope
                .dyn_clone()
                .reverse_lookup_ident(env, value)
        } else {
            Ok(None)
        }
    }

    fn lookup_invariant_limited<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ItemId,
        limit: u32,
    ) -> LookupInvariantResult {
        let result = self.local_lookup_invariant(env, invariant, limit);
        match result {
            Ok(inv) => Ok(inv),
            Err(LookupInvariantError::MightNotExist)
            | Err(LookupInvariantError::DefinitelyDoesNotExist)
            | Err(LookupInvariantError::Unresolved(..)) => {
                if let Some(parent) = self.parent() {
                    let parent_result = env
                        .get_item(parent)
                        .scope
                        .dyn_clone()
                        .lookup_invariant_limited(env, invariant, limit);
                    if parent_result == Err(LookupInvariantError::DefinitelyDoesNotExist) {
                        result
                    } else {
                        parent_result
                    }
                } else {
                    result
                }
            }
            Err(other) => Err(other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SPlain(pub ItemId);

impl Scope for SPlain {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _ident: &str,
    ) -> LookupIdentResult {
        Ok(None)
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _value: ItemId,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_lookup_invariant<'x>(
        &self,
        _env: &mut Environment<'x>,
        _invariant: ItemId,
        _limit: u32,
    ) -> LookupInvariantResult {
        Err(LookupInvariantError::DefinitelyDoesNotExist)
    }

    fn parent(&self) -> Option<ItemId> {
        Some(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct SRoot;

impl Scope for SRoot {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _ident: &str,
    ) -> LookupIdentResult {
        Ok(None)
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _value: ItemId,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ItemId,
        limit: u32,
    ) -> LookupInvariantResult {
        let truee = env.get_language_item("true");
        match env.discover_equal(invariant, truee, limit)? {
            Equal::Yes(l) => {
                if l.len() == 0 {
                    Ok(Invariant::new(truee, hashset![]))
                } else if l.len() > 0 {
                    Err(LookupInvariantError::DefinitelyDoesNotExist)
                } else {
                    unreachable!()
                }
            }
            Equal::NeedsHigherLimit => Err(LookupInvariantError::MightNotExist),
            Equal::Unknown | Equal::No => Err(LookupInvariantError::DefinitelyDoesNotExist),
        }
    }

    fn parent(&self) -> Option<ItemId> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct SPlaceholder;

impl Scope for SPlaceholder {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn is_placeholder(&self) -> bool {
        true
    }

    fn local_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _ident: &str,
    ) -> LookupIdentResult {
        unreachable!()
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _value: ItemId,
    ) -> ReverseLookupIdentResult {
        unreachable!()
    }

    fn local_lookup_invariant<'x>(
        &self,
        _env: &mut Environment<'x>,
        _invariant: ItemId,
        _limit: u32,
    ) -> LookupInvariantResult {
        unreachable!()
    }

    fn parent(&self) -> Option<ItemId> {
        unreachable!()
    }
}

#[derive(Clone, Debug)]
pub struct SWithParent<Base: Scope + Clone>(pub Base, pub ItemId);

impl<Base: Scope + Clone + 'static> Scope for SWithParent<Base> {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn is_placeholder(&self) -> bool {
        false
    }

    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> LookupIdentResult {
        self.0.local_lookup_ident(env, ident)
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ItemId,
    ) -> ReverseLookupIdentResult {
        self.0.local_reverse_lookup_ident(env, value)
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ItemId,
        limit: u32,
    ) -> LookupInvariantResult {
        self.0.local_lookup_invariant(env, invariant, limit)
    }

    fn parent(&self) -> Option<ItemId> {
        Some(self.1)
    }
}
