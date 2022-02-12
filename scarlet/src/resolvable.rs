use std::{collections::HashSet, fmt::Debug};

use crate::{
    constructs::{
        substitution::{CSubstitution, Substitutions},
        variable::{CVariable, Variable},
        ConstructDefinition, ConstructId,
    },
    environment::{Environment, UnresolvedConstructError},
    scope::Scope,
    shared::OrderedMap,
};

#[derive(Clone, Debug)]
pub enum ResolveError {
    UnresolvedConstruct(UnresolvedConstructError),
    InsufficientInvariants(String),
}

impl From<UnresolvedConstructError> for ResolveError {
    fn from(v: UnresolvedConstructError) -> Self {
        Self::UnresolvedConstruct(v)
    }
}

pub type ResolveResult<'x> = Result<ConstructDefinition<'x>, ResolveError>;

pub trait Resolvable<'x>: Debug {
    fn is_placeholder(&self) -> bool {
        false
    }
    fn dyn_clone(&self) -> BoxedResolvable<'x>;
    fn resolve(
        &self,
        env: &mut Environment<'x>,
        scope: Box<dyn Scope>,
        limit: u32,
    ) -> ResolveResult<'x>;
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
        env: &mut Environment<'x>,
        _scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult<'x> {
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

    fn resolve(
        &self,
        env: &mut Environment<'x>,
        scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult<'x> {
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

    fn resolve(
        &self,
        env: &mut Environment<'x>,
        scope: Box<dyn Scope>,
        limit: u32,
    ) -> ResolveResult<'x> {
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
                    return Err(partial_dep_error.into());
                } else {
                    eprintln!(
                        "BASE:\n{}\n",
                        env.show(self.base, self.base)
                            .unwrap_or(format!("Unresolved"))
                    );
                    panic!("No more dependencies left to substitute!");
                }
            }
            let dep = remaining_deps.pop_front().id;
            subs.insert_no_replace(dep, value);
        }

        let mut previous_subs = Substitutions::new();
        let mut justifications = Vec::new();
        for (target_id, value) in &subs {
            let target = env.get_variable(*target_id).clone();
            match target.can_be_assigned(*value, env, &previous_subs, limit)? {
                Ok(mut new_invs) => {
                    previous_subs.insert_no_replace(*target_id, *value);
                    justifications.append(&mut new_invs);
                }
                Err(err) => {
                    return Err(ResolveError::InsufficientInvariants(err));
                }
            }
        }

        let justification_deps: HashSet<_> = justifications
            .iter()
            .flat_map(|j| j.dependencies.iter())
            .collect();

        let mut invs = Vec::new();
        for inv in env.generated_invariants(base) {
            let mut new_inv = inv;
            for &&dep in &justification_deps {
                new_inv.dependencies.insert(dep);
            }
            new_inv.statement = env.substitute(new_inv.statement, &subs);
            invs.push(new_inv);
        }

        let csub = CSubstitution::new(self.base, subs, invs);
        Ok(ConstructDefinition::Resolved(Box::new(csub)))
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

    fn resolve(
        &self,
        env: &mut Environment<'x>,
        _scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult<'x> {
        let id = env.push_variable(Variable {
            id: None,
            invariants: self.invariants.clone(),
            dependencies: self.dependencies.clone(),
        });
        let con = CVariable::new(id);
        Ok(ConstructDefinition::Resolved(Box::new(con)))
    }
}
