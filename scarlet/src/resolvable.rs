use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use indexmap::{IndexMap, IndexSet};

use crate::{
    constructs::{as_variable, substitution::CSubstitution, Construct, ConstructId},
    environment::Environment,
    scope::{SPlain, Scope},
    shared::OrderedMap,
};

pub trait Resolvable<'x>: Debug {
    fn dyn_clone(&self) -> BoxedResolvable<'x>;
    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ConstructId;
}

pub type BoxedResolvable<'x> = Box<dyn Resolvable<'x> + 'x>;

#[derive(Clone, Debug)]
pub struct RPlaceholder;

impl<'x> Resolvable<'x> for RPlaceholder {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ConstructId {
        unreachable!()
    }
}

#[derive(Clone, Debug)]
pub struct RIdentifier<'x>(pub &'x str);

impl<'x> Resolvable<'x> for RIdentifier<'x> {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ConstructId {
        scope
            .lookup_ident(env, self.0)
            .expect(&format!("Cannot find what {} refers to", self.0))
    }
}

#[derive(Clone, Debug)]
pub struct RSubstitution<'x> {
    pub base: ConstructId,
    pub named_subs: Vec<(&'x str, ConstructId)>,
    pub anonymous_subs: Vec<ConstructId>,
}

impl<'x> Resolvable<'x> for RSubstitution<'x> {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ConstructId {
        let mut subs = OrderedMap::new();
        let this = env.push_placeholder(scope);
        let mut remaining_deps = env.get_dependencies(self.base);
        for &(name, value) in &self.named_subs {
            let target = RIdentifier(name).resolve(env, Box::new(SPlain(this)));
            if let Some(var) = as_variable(&**env.get_construct_definition(target)) {
                let index = remaining_deps.iter().position(|x| x == var);
                if let Some(index) = index {
                    remaining_deps.remove(index);
                }
                subs.insert_no_replace(var.clone(), value);
            } else {
                panic!("{} is a valid name, but it is not a variable", name)
            }
        }
        for &value in &self.anonymous_subs {
            if remaining_deps.len() == 0 {
                panic!("No more dependencies left to substitute!");
            }
            let dep = remaining_deps.remove(0);
            subs.insert_no_replace(dep, value);
        }
        let con = CSubstitution::new(self.base, subs);
        env.define_construct(this, con);
        this
    }
}
