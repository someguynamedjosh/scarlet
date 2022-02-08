use std::{cell::RefCell, collections::HashSet};

use super::{
    downcast_construct,
    variable::{CVariable, VariableId},
    Construct, ConstructDefinition, ConstructId, GenInvResult, Invariant,
};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies, DependencyError},
        CheckResult, DefEqualResult, Environment, UnresolvedConstructError,
    },
    impl_any_eq_for_construct,
    scope::Scope,
    shared::{OrderedMap, TripleBool},
};

pub type Substitutions = OrderedMap<VariableId, ConstructId>;
pub type NestedSubstitutions<'a> = OrderedMap<VariableId, SubExpr<'a>>;
type Justifications = Result<Vec<Invariant>, String>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubExpr<'a>(pub ConstructId, pub &'a NestedSubstitutions<'a>);

impl<'a> SubExpr<'a> {
    pub fn deps(&self, env: &mut Environment) -> DepResult {
        let mut result = Dependencies::new();
        let base = env.get_dependencies(self.0);
        for (target, value) in self.1.iter() {
            if base.contains_var(*target) {
                result.append(value.deps(env));
            }
        }
        result
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSubstitution(ConstructId, Substitutions, RefCell<Option<Justifications>>);

impl CSubstitution {
    pub fn new<'x>(base: ConstructId, subs: Substitutions) -> Self {
        Self(base, subs.clone(), RefCell::new(None))
    }

    pub(crate) fn new_unchecked(base: ConstructId, subs: Substitutions) -> Self {
        Self(base, subs, RefCell::new(Some(Ok(vec![]))))
    }

    pub fn base(&self) -> ConstructId {
        self.0
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.1
    }

    fn substitution_justifications(
        &self,
        env: &mut Environment,
    ) -> Result<&RefCell<Option<Justifications>>, UnresolvedConstructError> {
        Ok(if self.2.borrow().is_some() {
            &self.2
        } else {
            let just = self.create_substitution_justifications(env)?;
            *self.2.borrow_mut() = Some(just);
            &self.2
        })
    }

    fn create_substitution_justifications(
        &self,
        env: &mut Environment,
    ) -> Result<Justifications, UnresolvedConstructError> {
        let mut previous_subs = Substitutions::new();
        let mut invariants = Vec::new();
        for (target, value) in &self.1 {
            match env
                .get_variable(*target)
                .clone()
                .can_be_assigned(*value, env, &previous_subs)?
            {
                Ok(mut new_invs) => {
                    previous_subs.insert_no_replace(target.clone(), *value);
                    invariants.append(&mut new_invs)
                }
                Err(err) => {
                    panic!(
                        "THIS EXPRESSION:\n{}\nDOES NOT SATISFY THIS REQUIREMENT:\n{}",
                        env.show(*value, *value),
                        err
                    );
                }
            }
        }
        Ok(Ok(invariants))
    }

    fn invariants(&self, env: &mut Environment) -> GenInvResult {
        let mut invs = Vec::new();
        for inv in env.generated_invariants(self.0)? {
            let subbed_statement = env.substitute(inv.statement, &self.1);
            let mut new_deps: HashSet<_> = inv
                .dependencies
                .into_iter()
                .map(|d| env.substitute(d, &self.1))
                .collect();
            for inv in self
                .substitution_justifications(env)?
                .borrow()
                .iter()
                .flatten()
                .flatten()
            {
                for &dep in &inv.dependencies {
                    new_deps.insert(dep);
                }
            }
            invs.push(Invariant::new(subbed_statement, new_deps));
        }
        Ok(invs)
    }
}

impl_any_eq_for_construct!(CSubstitution);

impl Construct for CSubstitution {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(
        &self,
        env: &mut Environment<'x>,
        _this: ConstructId,
        _scope: Box<dyn Scope>,
    ) -> CheckResult {
        if let Err(err) = self
            .substitution_justifications(env)?
            .borrow()
            .as_ref()
            .unwrap()
        {
            eprintln!("{}", err);
            todo!("nice error");
        }
        Ok(())
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let mut deps = Dependencies::new();
        let base = env.get_dependencies(self.0);
        let base_error = base.error();
        for dep in base.as_variables() {
            if let Some((_, rep)) = self.1.iter().find(|(var, _)| *var == dep.id) {
                let replaced_deps = env.get_dependencies(*rep);
                let replaced_err = replaced_deps.error();
                for rdep in replaced_deps.into_variables() {
                    if !dep.swallow.contains(&rdep.id) {
                        deps.push_eager(rdep);
                    }
                }
                if let Some(err) = replaced_err {
                    deps.append(Dependencies::new_error(err));
                }
            } else {
                deps.push_eager(dep.clone());
            }
        }
        if let Some(err) = base_error {
            deps.append(Dependencies::new_error(err));
        }
        let sj = match self.substitution_justifications(env) {
            Ok(ok) => ok,
            Err(err) => {
                deps.append(Dependencies::new_error(err));
                return deps;
            }
        };
        for inv in sj.borrow().iter().flatten().flatten() {
            for &dep in &inv.dependencies {
                deps.append(env.get_dependencies(dep))
            }
        }
        deps
    }

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> GenInvResult {
        self.invariants(env)
    }

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        other: SubExpr,
        recursion_limit: u32,
    ) -> DefEqualResult {
        assert_ne!(recursion_limit, 0);
        let mut new_subs = subs.clone();
        for (target, value) in &self.1 {
            new_subs.insert_or_replace(target.clone(), SubExpr(*value, subs));
        }
        env.is_def_equal(SubExpr(self.0, &new_subs), other, recursion_limit - 1)
    }
}
