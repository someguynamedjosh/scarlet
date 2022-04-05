use std::fmt::Debug;

use crate::{
    constructs::ItemId,
    environment::{Environment, UnresolvedItemError},
};

pub type LookupIdentResult = Result<Option<ItemId>, UnresolvedItemError>;
pub type ReverseLookupIdentResult = Result<Option<String>, UnresolvedItemError>;

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

    fn parent(&self) -> Option<ItemId> {
        Some(self.1)
    }
}
