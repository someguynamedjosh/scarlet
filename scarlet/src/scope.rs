use std::fmt::Debug;

use maplit::hashset;

use crate::{
    constructs::{substitution::SubExpr, ConstructId, Invariant},
    environment::Environment,
    shared::TripleBool,
};

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
    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
        limit: u32,
    ) -> Option<Invariant>;
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
            if result.len() > 0 {
                return Some(result.to_owned());
            }
        }
        if let Some(parent) = self.parent() {
            env.get_construct(parent)
                .scope
                .dyn_clone()
                .reverse_lookup_ident(env, value)
        } else {
            None
        }
    }

    fn lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
    ) -> Option<Invariant> {
        for limit in 0..1024 {
            if let Some(inv) = self.local_lookup_invariant(env, invariant, limit) {
                return Some(inv);
            } else if let Some(parent) = self.parent() {
                let res = env
                    .get_construct(parent)
                    .scope
                    .dyn_clone()
                    .lookup_invariant(env, invariant);
                if res.is_some() {
                    return res;
                }
            }
        }
        None
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
        _limit: u32,
    ) -> Option<Invariant> {
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

    fn local_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        ident: &str,
    ) -> Option<ConstructId> {
        // } else if let Ok(_) = ident.parse() {
        //     todo!()
        None
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ConstructId,
    ) -> Option<String> {
        None
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
        limit: u32,
    ) -> Option<Invariant> {
        let truee = env.get_language_item("true");
        if env.is_def_equal(
            SubExpr(invariant, &Default::default()),
            SubExpr(truee, &Default::default()),
            limit,
        ) == TripleBool::True
        {
            Some(Invariant::new(truee, hashset![]))
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
        _limit: u32,
    ) -> Option<Invariant> {
        unreachable!()
    }

    fn parent(&self) -> Option<ConstructId> {
        unreachable!()
    }
}

#[derive(Clone, Debug)]
pub struct SWithParent<Base: Scope + Clone>(pub Base, pub ConstructId);

impl<Base: Scope + Clone + 'static> Scope for SWithParent<Base> {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn is_placeholder(&self) -> bool {
        false
    }

    fn local_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        ident: &str,
    ) -> Option<ConstructId> {
        self.0.local_lookup_ident(env, ident)
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ConstructId,
    ) -> Option<String> {
        self.0.local_reverse_lookup_ident(env, value)
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
        limit: u32,
    ) -> Option<Invariant> {
        self.0.local_lookup_invariant(env, invariant, limit)
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.1)
    }
}
