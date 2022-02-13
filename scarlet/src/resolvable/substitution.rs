use std::collections::HashSet;

use super::{BoxedResolvable, Resolvable, ResolveError, ResolveResult};
use crate::{
    constructs::{
        substitution::{CSubstitution, Substitutions},
        variable::CVariable,
        ConstructDefinition, ConstructId,
    },
    environment::Environment,
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
                    return Err(ResolveError::InvariantDeadEnd(err));
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
            for dep in std::mem::take(&mut new_inv.dependencies) {
                if let Some(var) = env.get_and_downcast_construct_definition::<CVariable>(dep)? {
                    if subs.contains_key(&var.get_id()) {
                        // Don't include any dependencies that are substituted with new values,
                        // because those are justified by the dependencies in
                        // justification_deps.
                        continue;
                    }
                }
                // However, if we don't substitute it, then we need to rely on the original
                // justification.
                new_inv.dependencies.insert(dep);
            }
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
