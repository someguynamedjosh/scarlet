use std::fmt::Debug;

use crate::{
    constructs::ConstructId,
    environment::Environment,
};

pub trait Scope: Debug {
    fn dyn_clone(&self) -> Box<dyn Scope>;
    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> Option<ConstructId>;
    fn parent(&self) -> Option<ConstructId>;

    fn lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> Option<ConstructId> {
        if let Some(result) = self.local_lookup_ident(env, ident) {
            Some(result)
        } else if let Some(parent) = self.parent() {
            env.get_construct(parent).scope.dyn_clone().lookup_ident(env, ident)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct SPlain(pub ConstructId);

impl Scope for SPlain {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(&self, _env: &mut Environment<'x>, _ident: &str) -> Option<ConstructId> {
        None
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

    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> Option<ConstructId> {
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

    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> Option<ConstructId> {
        unreachable!()
    }

    fn parent(&self) -> Option<ConstructId> {
        unreachable!()
    }
}
