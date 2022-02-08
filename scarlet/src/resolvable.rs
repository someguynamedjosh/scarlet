use std::fmt::Debug;

use crate::{
    constructs::{
        as_variable,
        substitution::{CSubstitution, Substitutions},
        variable::{CVariable, Variable, VariableId},
        ConstructDefinition, ConstructId,
    },
    environment::{Environment, UnresolvedConstructError},
    scope::Scope,
    shared::OrderedMap,
};

pub type ResolveResult<'x> = Result<ConstructDefinition<'x>, UnresolvedConstructError>;

pub trait Resolvable<'x>: Debug {
    fn is_placeholder(&self) -> bool {
        false
    }
    fn dyn_clone(&self) -> BoxedResolvable<'x>;
    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ResolveResult<'x>;
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

    fn resolve(&self, env: &mut Environment<'x>, _scope: Box<dyn Scope>) -> ResolveResult<'x> {
        eprintln!("{:#?}", env);
        unreachable!()
    }
}

#[derive(Clone, Debug)]
pub struct RIdentifier<'x>(pub &'x str);

impl<'x> Resolvable<'x> for RIdentifier<'x> {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ResolveResult<'x> {
        Ok(scope
            .lookup_ident(env, self.0)?
            .expect(&format!("Cannot find what {} refers to", self.0))
            .into())
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

    fn resolve(&self, env: &mut Environment<'x>, scope: Box<dyn Scope>) -> ResolveResult<'x> {
        let base = env.dereference(self.base)?;
        let base_scope = env.get_construct_scope(base).dyn_clone();
        let mut subs = OrderedMap::new();
        let mut remaining_deps = env.get_dependencies(self.base);
        for &(name, value) in &self.named_subs {
            let target = base_scope.lookup_ident(env, name)?.unwrap();
            if let Some(var) = env.get_and_downcast_construct_definition::<CVariable>(target)? {
                subs.insert_no_replace(var.get_id(), value);
                remaining_deps.remove(var.get_id());
            } else {
                panic!("{} is a valid name, but it is not a variable", name)
            }
        }
        for &value in &self.anonymous_subs {
            if remaining_deps.num_variables() == 0 {
                if let Some(partial_dep_error) = remaining_deps.error() {
                    return Err(partial_dep_error);
                } else {
                    eprintln!("BASE:\n{}\n", env.show(self.base, self.base));
                    panic!("No more dependencies left to substitute!");
                }
            }
            let dep = remaining_deps.pop_front().id;
            subs.insert_no_replace(dep, value);
        }
        Ok(ConstructDefinition::Resolved(Box::new(CSubstitution::new(
            self.base, subs,
        ))))
    }
}

#[derive(Clone, Debug)]
pub struct RVariable {
    pub invariants: Vec<ConstructId>,
    pub dependencies: Vec<ConstructId>,
}

impl<'x> Resolvable<'x> for RVariable {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(&self, env: &mut Environment<'x>, _scope: Box<dyn Scope>) -> ResolveResult<'x> {
        let id = env.push_variable(Variable {
            id: None,
            invariants: self.invariants.clone(),
            dependencies: self.dependencies.clone(),
        });
        let con = CVariable::new(id);
        Ok(ConstructDefinition::Resolved(Box::new(con)))
    }
}
