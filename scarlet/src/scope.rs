use std::fmt::Debug;

use crate::{constructs::ConstructId, environment::Environment, shared::TripleBool};

pub trait Scope: Debug {
    fn dyn_clone(&self) -> Box<dyn Scope>;

    fn is_placeholder(&self) -> bool {
        false
    }

    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str)
        -> Option<ConstructId>;
    fn local_reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ConstructId,
    ) -> Option<String>;
    fn local_lookup_invariant<'x>(&self, env: &mut Environment<'x>, invariant: ConstructId)
        -> bool;
    fn parent(&self) -> Option<ConstructId>;

    fn lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> Option<ConstructId> {
        if let Some(result) = self.local_lookup_ident(env, ident) {
            Some(result)
        } else if let Some(parent) = self.parent() {
            env.get_construct(parent)
                .scope
                .dyn_clone()
                .lookup_ident(env, ident)
        } else {
            None
        }
    }

    fn reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ConstructId,
    ) -> Option<String> {
        if let Some(result) = self.local_reverse_lookup_ident(env, value) {
            Some(result.to_owned())
        } else if let Some(parent) = self.parent() {
            env.get_construct(parent)
                .scope
                .dyn_clone()
                .reverse_lookup_ident(env, value)
        } else {
            None
        }
    }

    fn lookup_invariant<'x>(&self, env: &mut Environment<'x>, invariant: ConstructId) -> bool {
        if self.local_lookup_invariant(env, invariant) {
            true
        } else if let Some(parent) = self.parent() {
            env.get_construct(parent)
                .scope
                .dyn_clone()
                .lookup_invariant(env, invariant)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct SPlain(pub ConstructId);

impl Scope for SPlain {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _ident: &str,
    ) -> Option<ConstructId> {
        None
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _value: ConstructId,
    ) -> Option<String> {
        None
    }

    fn local_lookup_invariant<'x>(
        &self,
        _env: &mut Environment<'x>,
        _invariant: ConstructId,
    ) -> bool {
        false
    }

    fn parent(&self) -> Option<ConstructId> {
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
        env: &mut Environment<'x>,
        ident: &str,
    ) -> Option<ConstructId> {
        if ident == "true" {
            Some(env.get_builtin_item("true").into())
        } else if ident == "false" {
            Some(env.get_builtin_item("false").into())
        // } else if let Ok(_) = ident.parse() {
        //     todo!()
        } else {
            None
        }
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ConstructId,
    ) -> Option<String> {
        if value == env.get_builtin_item("true") {
            Some("true".to_owned())
        } else if value == env.get_builtin_item("false") {
            Some("false".to_owned())
        } else {
            None
        }
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
    ) -> bool {
        let truee = env.get_builtin_item("true");
        env.is_def_equal(invariant, truee) == TripleBool::True
    }

    fn parent(&self) -> Option<ConstructId> {
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
    ) -> Option<ConstructId> {
        unreachable!()
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        _env: &mut Environment<'x>,
        _value: ConstructId,
    ) -> Option<String> {
        unreachable!()
    }

    fn local_lookup_invariant<'x>(
        &self,
        _env: &mut Environment<'x>,
        _invariant: ConstructId,
    ) -> bool {
        unreachable!()
    }

    fn parent(&self) -> Option<ConstructId> {
        unreachable!()
    }
}
