use std::fmt::Debug;

use crate::{
    constructs::ConstructId,
    environment::Environment,
    shared::{Id, Pool},
};

pub type ScopeId = Id<'S'>;
pub type ScopePool = Pool<AnnotatedScope, 'S'>;
#[derive(Debug)]
pub struct AnnotatedScope {
    pub scope: Box<dyn Scope>,
    pub parent: Option<ScopeId>,
}

impl Clone for AnnotatedScope {
    fn clone(&self) -> Self {
        Self {
            scope: self.scope.dyn_clone(),
            parent: self.parent,
        }
    }
}

impl AnnotatedScope {
    pub fn lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> Option<ConstructId> {
        if let Some(result) = self.scope.lookup_ident(env, ident) {
            Some(result)
        } else if let Some(parent) = self.parent {
            env.get_scope(parent).clone().lookup_ident(env, ident)
        } else {
            None
        }
    }
}

pub trait Scope: Debug {
    fn dyn_clone(&self) -> Box<dyn Scope>;
    fn lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> Option<ConstructId>;
}

#[derive(Debug, Clone)]
pub struct SEmpty;

impl Scope for SEmpty {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn lookup_ident<'x>(&self, _env: &mut Environment<'x>, _ident: &str) -> Option<ConstructId> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct SRoot;

impl Scope for SRoot {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> Option<ConstructId> {
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
}
