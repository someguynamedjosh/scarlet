use std::fmt::Debug;

use crate::{
    constructs::{
        as_variable,
        substitution::CSubstitution,
        variable::{CVariable, VariableId},
        ConstructDefinition, ConstructId,
    },
    environment::Environment,
    scope::Scope,
    shared::OrderedMap,
};

pub trait Resolvable<'x>: Debug {
    fn is_placeholder(&self) -> bool {
        false
    }
    fn dyn_clone(&self) -> BoxedResolvable<'x>;
    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ConstructDefinition<'x>;
}

pub type BoxedResolvable<'x> = Box<dyn Resolvable<'x> + 'x>;

#[derive(Clone, Debug)]
pub struct RPlaceholder;

impl<'x> Resolvable<'x> for RPlaceholder {
    fn is_placeholder(&self) -> bool {
        true
    }

    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        _env: &mut Environment<'x>,
        _scope: Box<dyn Scope>,
    ) -> ConstructDefinition<'x> {
        unreachable!()
    }
}

#[derive(Clone, Debug)]
pub struct RIdentifier<'x>(pub &'x str);

impl<'x> Resolvable<'x> for RIdentifier<'x> {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ConstructDefinition<'x> {
        scope
            .lookup_ident(env, self.0)
            .expect(&format!("Cannot find what {} refers to", self.0))
            .into()
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

    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ConstructDefinition<'x> {
        let base_scope = env.get_original_construct_scope(self.base).dyn_clone();
        let mut subs = OrderedMap::new();
        let mut remaining_deps = env.get_dependencies(self.base);
        for &(name, value) in &self.named_subs {
            let target = base_scope.lookup_ident(env, name).unwrap();
            println!("{} is {:?}", name, target);
            if let Some(var) = as_variable(&**env.get_reduced_construct_definition(target)) {
                remaining_deps.remove(var);
                subs.insert_no_replace(var.clone(), value);
            } else {
                panic!("{} is a valid name, but it is not a variable", name)
            }
        }
        for &value in &self.anonymous_subs {
            if remaining_deps.num_variables() == 0 {
                panic!("No more dependencies left to substitute!");
            }
            let dep = remaining_deps.pop_front();
            subs.insert_no_replace(dep, value);
        }
        ConstructDefinition::Resolved(Box::new(CSubstitution::new(self.base, subs)))
    }
}

#[derive(Clone, Debug)]
pub struct RVariable {
    pub id: VariableId,
    pub invariants: Vec<ConstructId>,
    pub substitutions: Vec<ConstructId>,
}

impl<'x> Resolvable<'x> for RVariable {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        _env: &mut Environment<'x>,
        _scope: Box<dyn Scope>,
    ) -> ConstructDefinition<'x> {
        let con = CVariable::new(self.id, self.invariants.clone(), self.substitutions.clone());
        ConstructDefinition::Resolved(Box::new(con))
    }
}
