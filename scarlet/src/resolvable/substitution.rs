use super::{BoxedResolvable, Resolvable, ResolveError, ResolveResult};
use crate::{
    constructs::{
        substitution::{CSubstitution, Substitutions},
        variable::CVariable,
        ItemDefinition, ItemId,
    },
    environment::{dependencies::Dependencies, Environment},
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
        _limit: u32,
    ) -> ResolveResult<'x> {
        let base = env.dereference(self.base)?;
        let base_scope = env.get_item_scope(base).dyn_clone();
        let mut subs = OrderedMap::new();
        let mut remaining_deps = env.get_dependencies(self.base);

        self.resolve_named_subs(base_scope, env, &mut subs, &mut remaining_deps)?;
        self.resolve_anonymous_subs(remaining_deps, env, &mut subs)?;
        resolve_dep_subs(&mut subs, env);

        let csub = CSubstitution::new(self.base, subs);
        Ok(ItemDefinition::Resolved(Box::new(csub)))
    }
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
