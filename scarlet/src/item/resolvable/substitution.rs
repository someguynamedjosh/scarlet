use maplit::hashset;

use super::{BoxedResolvable, Resolvable, ResolveError, ResolveResult};
use crate::{
    diagnostic::{Diagnostic, Position},
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        definitions::{
            substitution::{DSubstitution, Substitutions},
            variable::{DVariable, Variable},
        },
        dependencies::{Dcc, Dependencies},
        invariants::{PredicateSet, PredicatesResult},
        util::unchecked_substitution,
        ContainmentType, ItemDefinition, ItemPtr,
    },
    scope::Scope,
    shared::OrderedMap,
    util::PtrExtension,
};

#[derive(Clone, Debug)]
pub struct RSubstitution {
    pub base: ItemPtr,
    pub position: Position,
    pub named_subs: Vec<(Position, String, ItemPtr)>,
    pub named_proofs: Vec<(Position, String, ItemPtr)>,
    pub anonymous_subs: Vec<ItemPtr>,
}

impl PartialEq for RSubstitution {
    fn eq(&self, other: &Self) -> bool {
        if !self.base.is_same_instance_as(&other.base)
            || self.named_subs.len() != other.named_subs.len()
            || self.anonymous_subs.len() != other.anonymous_subs.len()
        {
            return false;
        }
        'next_sub: for (_, key, value) in &self.named_subs {
            for (_, other_key, other_value) in &other.named_subs {
                if key == other_key {
                    if value.is_same_instance_as(other_value) {
                        continue 'next_sub;
                    } else {
                        // The target is replaced with something else.
                        return false;
                    }
                }
            }
            // There is no matching substitution.
            return false;
        }
        for (sub, other_sub) in self.anonymous_subs.iter().zip(other.anonymous_subs.iter()) {
            if !sub.is_same_instance_as(other_sub) {
                return false;
            }
        }
        true
    }
}

impl_any_eq_from_regular_eq!(RSubstitution);

impl Resolvable for RSubstitution {
    fn dyn_clone(&self) -> BoxedResolvable {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        env: &mut Environment,
        this: ItemPtr,
        _scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult {
        let base = self.base.dereference_resolved()?;
        let base_scope = base.clone_scope();
        let mut subs = OrderedMap::new();
        let mut remaining_deps = self.base.get_dependencies();
        let total_dep_count = remaining_deps.num_variables();

        self.resolve_named_subs(&base, base_scope, env, &mut subs, &mut remaining_deps)?;
        self.resolve_named_proofs(&base, env, &mut subs, &mut remaining_deps)?;
        self.resolve_anonymous_subs(total_dep_count, remaining_deps, env, &mut subs)?;
        resolve_dep_subs(&mut subs)?;

        DSubstitution::new_into(&this, self.base.ptr_clone(), subs)?;
        ResolveResult::Ok
    }

    fn estimate_dependencies(&self, ctx: &mut Dcc, affects_return_value: bool) -> Dependencies {
        let mut result = Dependencies::new();
        for (_, _, arg) in &self.named_subs {
            result.append(ctx.get_dependencies(arg, affects_return_value));
        }
        for arg in &self.anonymous_subs {
            result.append(ctx.get_dependencies(arg, affects_return_value));
        }
        result
    }

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)> {
        let mut result = vec![(ContainmentType::Computational, &self.base)];
        for (_, _, value) in &self.named_subs {
            result.push((ContainmentType::Computational, value));
        }
        for (_, _, proof) in &self.named_proofs {
            result.push((ContainmentType::Definitional, proof));
        }
        for value in &self.anonymous_subs {
            result.push((ContainmentType::Computational, value));
        }
        result
    }
}

/// Turns things like fx[fx IS gy] to fx[fx IS gy[y IS x]] so that the
/// dependencies match.
fn resolve_dep_subs(subs: &mut Substitutions) -> Result<(), ResolveError> {
    let value_subs: Substitutions = subs
        .iter()
        .filter(|(target, _)| target.borrow().required_theorem().is_none())
        .cloned()
        .collect();
    for (target, value) in subs {
        let mut dep_subs = Substitutions::new();
        let value_deps = value.get_dependencies();
        let mut value_deps_iter = value_deps.as_variables();
        let mut value_reqs_iter = value_deps.as_requirements();
        for target_dep in target.borrow().get_dependencies() {
            let target_dep = target_dep.dereference();
            let target_dep = target_dep
                .downcast_resolved_definition::<DVariable>()?
                .unwrap();
            let existing_dep = if target_dep
                .get_variable()
                .borrow()
                .required_theorem()
                .is_some()
            {
                value_reqs_iter.next().map(|x| &x.var)
            } else {
                value_deps_iter.next().map(|x| &x.var)
            };
            if let Some(existing_dep) = existing_dep {
                if !existing_dep.is_same_instance_as(target_dep.get_variable()) {
                    let mut desired_dep = target_dep.get_variable().borrow().item().ptr_clone();
                    if target_dep
                        .get_variable()
                        .borrow()
                        .required_theorem()
                        .is_some()
                    {
                        drop(target_dep);
                        desired_dep = unchecked_substitution(desired_dep, value_subs.clone())?;
                    }
                    dep_subs.insert_no_replace(existing_dep.ptr_clone(), desired_dep);
                }
            } else if let Some(err) = value_deps.error() {
                return Err(err.clone().into());
            }
        }
        if dep_subs.len() > 0 {
            *value = unchecked_substitution(value.ptr_clone(), dep_subs)?;
        }
    }
    Ok(())
}

impl RSubstitution {
    fn resolve_anonymous_subs(
        &self,
        total_dep_count: usize,
        mut remaining_deps: Dependencies,
        env: &mut Environment,
        subs: &mut Substitutions,
    ) -> Result<(), ResolveError> {
        for value in &self.anonymous_subs {
            if remaining_deps.num_variables() == 0 {
                if let Some(partial_dep_error) = remaining_deps.error() {
                    return Err(partial_dep_error.clone().into());
                } else {
                    return Err(Diagnostic::new()
                        .with_text_error(format!("This substitution has too many arguments:"))
                        .with_source_code_block_error(self.position)
                        .with_text_info(format!(
                            "The base only has {} dependencies:",
                            total_dep_count
                        ))
                        .with_item_info(&self.base, &self.base, env)
                        .into());
                }
            }
            let dep = remaining_deps.pop_front().var;
            subs.insert_no_replace(dep, value.ptr_clone());
        }
        Ok(())
    }

    fn resolve_named_subs(
        &self,
        base: &ItemPtr,
        base_scope: Box<dyn Scope>,
        env: &mut Environment,
        subs: &mut Substitutions,
        remaining_deps: &mut Dependencies,
    ) -> Result<(), ResolveError> {
        for (position, name, value) in &self.named_subs {
            let target = base_scope.lookup_ident(&name)?.ok_or_else(|| {
                Diagnostic::new()
                    .with_text_error(format!(
                        concat!(
                            "The name \"{}\" does not refer to a variable ",
                            "in the scope of the function being called:"
                        ),
                        name
                    ))
                    .with_source_code_block_error(*position)
                    .with_text_info(format!("The function is defined here:"))
                    .with_item_info(base, base, env)
            })?;
            if let Some(var) = target
                .dereference()
                .downcast_resolved_definition::<DVariable>()?
            {
                subs.insert_no_replace(var.get_variable().ptr_clone(), value.ptr_clone());
                remaining_deps.remove(var.get_variable());
            } else {
                return Err(Diagnostic::new()
                    .with_text_error(format!(
                        "{} is used as a variable here but it is actually something else:",
                        name
                    ))
                    .with_source_code_block_error(*position)
                    .with_text_info(format!("{} is actually defined as follows:", name))
                    .with_item_info(&target, value, env)
                    .into());
            }
            drop(target);
        }
        Ok(())
    }

    fn resolve_named_proofs(
        &self,
        base: &ItemPtr,
        env: &mut Environment,
        subs: &mut Substitutions,
        remaining_deps: &mut Dependencies,
    ) -> Result<(), ResolveError> {
        for (position, statement_text, value) in &self.named_proofs {
            let target = remaining_deps
                .as_requirements()
                .find(|req| &req.statement_text == statement_text)
                .ok_or_else(|| {
                    Diagnostic::new()
                        .with_text_error(format!(
                            concat!("The function has no requirement whose text matches \"{}\"",),
                            statement_text
                        ))
                        .with_source_code_block_error(*position)
                        .with_text_info(format!("The function is defined here:"))
                        .with_item_info(base, base, env)
                })?;
            let var = target.var.ptr_clone();
            remaining_deps.remove(&var);
            subs.insert_no_replace(var.ptr_clone(), value.ptr_clone());
        }
        Ok(())
    }
}
