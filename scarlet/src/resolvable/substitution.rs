use std::collections::HashSet;

use super::{BoxedResolvable, Resolvable, ResolveError, ResolveResult};
use crate::{
    constructs::{
        substitution::{CSubstitution, Substitutions},
        variable::CVariable,
        ConstructDefinition, ConstructId,
    },
    environment::{dependencies::Dependencies, Environment},
    scope::Scope,
    shared::OrderedMap,
};

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
        _scope: Box<dyn Scope>,
        limit: u32,
    ) -> ResolveResult<'x> {
        let base = env.dereference(self.base)?;
        let base_scope = env.get_construct_scope(base).dyn_clone();
        let mut subs = OrderedMap::new();
        let mut remaining_deps = env.get_dependencies(self.base);

        self.resolve_named_subs(base_scope, env, &mut subs, &mut remaining_deps)?;
        self.resolve_anonymous_subs(remaining_deps, env, &mut subs)?;
        resolve_dep_subs(&mut subs, env);

        let justifications = find_justifications(&subs, env, limit)?;
        let justification_deps = extract_invariant_dependencies(justifications);
        let invs = create_invariants(env, base, &subs, justification_deps)?;

        let csub = CSubstitution::new(self.base, subs, invs);
        Ok(ConstructDefinition::Resolved(Box::new(csub)))
    }
}

fn create_invariants(
    env: &mut Environment,
    base: ConstructId,
    subs: &Substitutions,
    justification_deps: HashSet<ConstructId>,
) -> Result<Vec<crate::environment::invariants::Invariant>, ResolveError> {
    let mut invs = Vec::new();
    for inv in env.generated_invariants(base) {
        let mut new_inv = inv;
        for dep in std::mem::take(&mut new_inv.dependencies) {
            if let Some(var) = env.get_and_downcast_construct_definition::<CVariable>(dep)? {
                if subs.contains_key(&var.get_id()) {
                    // Don't include any dependencies that are substituted with new values,
                    // because those are replaced by the dependencies in
                    // justification_deps. When we substitute something, we want to use the
                    // substituted thing as justification, not the thing that was substituted for
                    // and is now gone.
                    continue;
                }
            }
            // However, if we don't substitute it, then we need to rely on the original
            // justification.
            new_inv.dependencies.insert(dep);
        }
        // Apply the substitutions to the statement the invariant is making.
        new_inv.statement = env.substitute(new_inv.statement, subs);
        // The substituted invariant is also justified by all the justifications for the
        // original substitution. E.G. if we substitute a with b, then any invariant
        // about a is now an invariant about b, justified by the fact that we can
        // replace a with b.
        for &dep in &justification_deps {
            new_inv.dependencies.insert(dep);
        }
        invs.push(new_inv);
    }
    Ok(invs)
}

fn extract_invariant_dependencies(
    justifications: Vec<crate::environment::invariants::Invariant>,
) -> HashSet<ConstructId> {
    justifications
        .iter()
        .flat_map(|j| j.dependencies.iter().copied())
        .collect()
}

/// Finds invariants that confirm the substitutions we're performing are legal.
/// For example, an_int[an_int IS something] would need an invariant of the form
/// `(an_int FROM I32)[an_int IS something]`.
fn find_justifications(
    subs: &Substitutions,
    env: &mut Environment,
    limit: u32,
) -> Result<Vec<crate::environment::invariants::Invariant>, ResolveError> {
    let mut justifications = Vec::new();
    let mut previous_subs = Substitutions::new();
    for (target_id, value) in subs {
        let target = env.get_variable(*target_id).clone();
        match target.can_be_assigned(*value, env, &previous_subs, limit)? {
            Ok(mut new_invs) => {
                previous_subs.insert_no_replace(*target_id, *value);
                justifications.append(&mut new_invs);
            }
            Err(err) => {
                return Err(ResolveError::InvariantDeadEnd(err));
            }
        }
    }
    Ok(justifications)
}

/// Turns things like fx[fx IS gy] to fx[fx IS gy[y IS x]] so that the
/// dependencies match.
fn resolve_dep_subs(subs: &mut Substitutions, env: &mut Environment) {
    for (target, value) in subs {
        let mut dep_subs = Substitutions::new();
        let target = env.get_variable(*target).clone();
        let value_deps = env.get_dependencies(*value);
        let mut value_deps = value_deps.as_variables();
        for dep in target.get_dependencies() {
            for desired_dep in env.get_dependencies(*dep).as_variables() {
                // We want to convert a dependency in the value to the
                // dependency required by the variable it is assigned to.
                if let Some(existing_dep) = value_deps.next() {
                    if existing_dep.id != desired_dep.id {
                        let desired_dep = env.get_variable(desired_dep.id).construct.unwrap();
                        dep_subs.insert_no_replace(existing_dep.id, desired_dep);
                    }
                }
            }
        }
        if dep_subs.len() > 0 {
            *value = env.substitute(*value, &dep_subs);
        }
    }
}

impl<'x> RSubstitution<'x> {
    fn resolve_anonymous_subs(
        &self,
        mut remaining_deps: Dependencies,
        env: &mut Environment,
        subs: &mut Substitutions,
    ) -> Result<(), ResolveError> {
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
        Ok(())
    }

    fn resolve_named_subs(
        &self,
        base_scope: Box<dyn Scope>,
        env: &mut Environment,
        subs: &mut Substitutions,
        remaining_deps: &mut Dependencies,
    ) -> Result<(), ResolveError> {
        for &(name, value) in &self.named_subs {
            let target = base_scope.lookup_ident(env, name)?.unwrap();
            if let Some(var) = env.get_and_downcast_construct_definition::<CVariable>(target)? {
                subs.insert_no_replace(var.get_id(), value);
                remaining_deps.remove(var.get_id());
            } else {
                panic!("{} is a valid name, but it is not a variable", name)
            }
        }
        Ok(())
    }
}
