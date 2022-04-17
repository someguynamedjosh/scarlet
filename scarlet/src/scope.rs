use std::fmt::Debug;

use crate::{
    environment::Environment,
    item::{resolvable::UnresolvedItemError, ItemPtr, InvariantSetPtr},
};

pub type LookupIdentResult = Result<Option<ItemPtr>, UnresolvedItemError>;
pub type ReverseLookupIdentResult = Result<Option<String>, UnresolvedItemError>;
pub type LookupInvariantResult = Result<InvariantSetPtr, LookupInvariantError>;

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

    fn local_lookup_ident(&self, env: &mut Environment, ident: &str) -> LookupIdentResult;
    fn local_reverse_lookup_ident(
        &self,
        env: &mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult;
    fn local_get_invariant_sets(&self, env: &mut Environment) -> Vec<InvariantSetPtr>;
    fn parent(&self) -> Option<ItemPtr>;

    fn lookup_ident(&self, env: &mut Environment, ident: &str) -> LookupIdentResult {
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

    fn reverse_lookup_ident(
        &self,
        env: &mut Environment,
        value: ItemPtr,
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

    fn get_invariant_sets(&self, env: &mut Environment) -> Vec<InvariantSetPtr> {
        let mut result = self.local_get_invariant_sets(env);
        if let Some(parent) = self.parent() {
            let parent_scope = env.get_item(parent).scope.dyn_clone();
            result.append(&mut parent_scope.get_invariant_sets(env));
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct SPlain(pub ItemPtr);

impl Scope for SPlain {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, _env: &mut Environment, _ident: &str) -> LookupIdentResult {
        Ok(None)
    }

    fn local_reverse_lookup_ident(
        &self,
        _env: &mut Environment,
        _value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_get_invariant_sets(&self, _env: &mut Environment) -> Vec<InvariantSetPtr> {
        vec![]
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct SRoot;

impl Scope for SRoot {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, _env: &mut Environment, _ident: &str) -> LookupIdentResult {
        Ok(None)
    }

    fn local_reverse_lookup_ident(
        &self,
        _env: &mut Environment,
        _value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_get_invariant_sets(&self, env: &mut Environment) -> Vec<InvariantSetPtr> {
        vec![]
    }

    fn parent(&self) -> Option<ItemPtr> {
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

    fn local_lookup_ident(&self, _env: &mut Environment, _ident: &str) -> LookupIdentResult {
        unreachable!()
    }

    fn local_reverse_lookup_ident(
        &self,
        _env: &mut Environment,
        _value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        unreachable!()
    }

    fn local_get_invariant_sets(&self, _env: &mut Environment) -> Vec<InvariantSetPtr> {
        unreachable!()
    }

    fn parent(&self) -> Option<ItemPtr> {
        unreachable!()
    }
}

#[derive(Clone, Debug)]
pub struct SWithParent<Base: Scope + Clone>(pub Base, pub ItemPtr);

impl<Base: Scope + Clone + 'static> Scope for SWithParent<Base> {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn is_placeholder(&self) -> bool {
        false
    }

    fn local_lookup_ident(&self, env: &mut Environment, ident: &str) -> LookupIdentResult {
        self.0.local_lookup_ident(env, ident)
    }

    fn local_reverse_lookup_ident(
        &self,
        env: &mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        self.0.local_reverse_lookup_ident(env, value)
    }

    fn local_get_invariant_sets(&self, env: &mut Environment) -> Vec<InvariantSetPtr> {
        self.0.local_get_invariant_sets(env)
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.1)
    }
}
