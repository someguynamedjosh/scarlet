use std::fmt::Debug;

use crate::{
    environment::Environment,
    item::{invariants::InvariantSetPtr, resolvable::UnresolvedItemError, ItemPtr},
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

    fn local_lookup_ident(&self, ident: &str) -> LookupIdentResult;
    fn local_reverse_lookup_ident(
        &self,
        env: &mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult;
    fn local_get_invariant_sets(&self) -> Vec<InvariantSetPtr>;
    fn parent(&self) -> Option<ItemPtr>;

    fn lookup_ident(&self, ident: &str) -> LookupIdentResult {
        if let Some(result) = self.local_lookup_ident(ident)? {
            Ok(Some(result))
        } else if let Some(parent) = self.parent() {
            parent.borrow().scope.lookup_ident(ident)
        } else {
            Ok(None)
        }
    }

    fn reverse_lookup_ident(
        &self,
        env: &mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        if let Some(result) = self.local_reverse_lookup_ident(env, value.ptr_clone())? {
            if result.len() > 0 {
                return Ok(Some(result.to_owned()));
            }
        }
        if let Some(parent) = self.parent() {
            parent
                .borrow()
                .scope
                .reverse_lookup_ident(env, value.ptr_clone())
        } else {
            Ok(None)
        }
    }

    fn get_invariant_sets(&self) -> Vec<InvariantSetPtr> {
        let mut result = self.local_get_invariant_sets();
        if let Some(parent) = self.parent() {
            let parent_scope = &parent.borrow().scope;
            result.append(&mut parent_scope.get_invariant_sets());
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

    fn local_lookup_ident(&self, _ident: &str) -> LookupIdentResult {
        Ok(None)
    }

    fn local_reverse_lookup_ident(
        &self,
        _env: &mut Environment,
        _value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_get_invariant_sets(&self) -> Vec<InvariantSetPtr> {
        vec![]
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.0.ptr_clone())
    }
}

#[derive(Debug, Clone)]
pub struct SRoot;

impl Scope for SRoot {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, _ident: &str) -> LookupIdentResult {
        Ok(None)
    }

    fn local_reverse_lookup_ident(
        &self,
        _env: &mut Environment,
        _value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_get_invariant_sets(&self) -> Vec<InvariantSetPtr> {
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

    fn local_lookup_ident(&self, _ident: &str) -> LookupIdentResult {
        unreachable!()
    }

    fn local_reverse_lookup_ident(
        &self,
        _env: &mut Environment,
        _value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        unreachable!()
    }

    fn local_get_invariant_sets(&self) -> Vec<InvariantSetPtr> {
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

    fn local_lookup_ident(&self, ident: &str) -> LookupIdentResult {
        self.0.local_lookup_ident(ident)
    }

    fn local_reverse_lookup_ident(
        &self,
        env: &mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        self.0.local_reverse_lookup_ident(env, value)
    }

    fn local_get_invariant_sets(&self) -> Vec<InvariantSetPtr> {
        self.0.local_get_invariant_sets()
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.1.ptr_clone())
    }
}
