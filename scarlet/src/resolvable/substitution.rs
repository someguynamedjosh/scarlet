use std::collections::HashSet;

use itertools::Itertools;
use maplit::hashset;

use super::{BoxedResolvable, Resolvable, ResolveError, ResolveResult};
use crate::{
    constructs::{
        substitution::{CSubstitution, Substitutions},
        variable::CVariable,
        ItemDefinition, ItemId,
    },
    environment::{dependencies::Dependencies, invariants::InvariantSet, Environment},
    scope::Scope,
    shared::OrderedMap,
};

#[derive(Clone, Debug)]
pub struct RSubstitution<'x> {
    pub base: ItemId,
    pub named_subs: Vec<(&'x str, ItemId)>,
    pub anonymous_subs: Vec<ItemId>,
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
    ) -> ResolveResult {
        let base = env.dereference_no_unresolved_error(self.base);
        let base_scope = env.get_item_scope(base).dyn_clone();
        let mut subs = OrderedMap::new();
        let mut remaining_deps = env.get_dependencies(self.base);

        self.resolve_named_subs(base_scope, env, &mut subs, &mut remaining_deps)?;
        self.resolve_anonymous_subs(remaining_deps, env, &mut subs)?;
        resolve_dep_subs(&mut subs, env);

        let justifications = make_justification_statements(&subs, env, limit)?;
        let invs = create_invariants(env, base, &subs, justifications);
        let invs = env.push_invariant_set(invs);
        let csub = CSubstitution::new(self.base, subs, invs);
        ResolveResult::Ok(ItemDefinition::Resolved(Box::new(csub)))
    }

    fn estimate_dependencies(&self, env: &mut Environment) -> Dependencies {
        let mut result = Dependencies::new();
        for &(_, arg) in &self.named_subs {
            result.append(env.get_dependencies(arg));
        }
        for &arg in &self.anonymous_subs {
            result.append(env.get_dependencies(arg));
        }
        result
    }
}

fn create_invariants(
    env: &mut Environment,
    base: ItemId,
    subs: &Substitutions,
    justifications: Vec<ItemId>,
) -> InvariantSet {
    let mut invs = Vec::new();
    let set_id = env.generated_invariants(base);
    for inv in env
        .get_invariant_set(set_id)
        .statements()
        .into_iter()
        .cloned()
        .collect_vec()
    {
        // Apply the substitutions to the statement the invariant is making.
        let new_inv = env.substitute_unchecked(inv, subs);
        invs.push(new_inv);
    }
    InvariantSet::new(base, invs, justifications, hashset![])
}

/// Finds invariants that confirm the substitutions we're performing are legal.
/// For example, an_int[an_int IS something] would need an invariant of the form
/// `(an_int FROM I32)[an_int IS something]`.
fn make_justification_statements(
    subs: &Substitutions,
    env: &mut Environment,
    limit: u32,
) -> Result<Vec<ItemId>, ResolveError> {
    let mut justifications = Vec::new();
    let mut previous_subs = Substitutions::new();
    for (target_id, value) in subs {
        let target = env.get_variable(*target_id).clone();
        justifications.append(&mut target.assignment_justifications(
            *value,
            env,
            &previous_subs,
            limit,
        ));
        previous_subs.insert_no_replace(*target_id, *value);
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
                        let desired_dep = env.get_variable(desired_dep.id).item.unwrap();
                        dep_subs.insert_no_replace(existing_dep.id, desired_dep);
                    }
                }
            }
        }
        if dep_subs.len() > 0 {
            *value = env.substitute_unchecked(*value, &dep_subs);
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
                    eprintln!("BASE:\n{}\n", env.show(self.base, self.base));
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
