use std::{cell::RefCell, collections::HashSet};

use super::{
    downcast_construct, variable::CVariable, Construct, ConstructDefinition, ConstructId, Invariant,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    scope::Scope,
    shared::{OrderedMap, TripleBool},
};

pub type Substitutions = OrderedMap<CVariable, ConstructId>;
pub type NestedSubstitutions<'a> = OrderedMap<CVariable, SubExpr<'a>>;
type Justifications = Result<Vec<Invariant>, String>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubExpr<'a>(pub ConstructId, pub &'a NestedSubstitutions<'a>);

impl<'a> SubExpr<'a> {
    pub fn deps(&self, env: &mut Environment) -> Dependencies {
        let mut result = Dependencies::new();
        let base = env.get_dependencies(self.0);
        for (target, value) in self.1.iter() {
            if base.contains(target) {
                result.append(value.deps(env));
            }
        }
        result
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSubstitution(ConstructId, Substitutions, RefCell<Option<Justifications>>);

impl CSubstitution {
    pub fn new<'x>(base: ConstructId, subs: Substitutions, env: &mut Environment) -> Self {
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
    ) -> &RefCell<Option<Justifications>> {
        if self.2.borrow().is_some() {
            return &self.2;
        } else {
            let just = self.create_substitution_justifications(env);
            *self.2.borrow_mut() = Some(just);
            &self.2
        }
    }

    fn create_substitution_justifications(&self, env: &mut Environment) -> Justifications {
        let mut previous_subs = Substitutions::new();
        let mut invariants = Vec::new();
        for (target, value) in &self.1 {
            match target.can_be_assigned(*value, env, &previous_subs) {
                Ok(mut new_invs) => {
                    previous_subs.insert_no_replace(target.clone(), *value);
                    invariants.append(&mut new_invs)
                }
                Err(err) => {
                    return Err(format!(
                        "THIS EXPRESSION:\n{}\nASSIGNED TO:\n{}\nDOES NOT SATISFY THIS REQUIREMENT:\n{}",
                        env.show(*value, *value),
                        env.show_var(target, *value),
                        err
                    ));
                }
            }
        }
        Ok(invariants)
    }

    fn invariants(&self, env: &mut Environment) -> Vec<Invariant> {
        let mut invs = Vec::new();
        for inv in env.generated_invariants(self.0) {
            let subbed_statement = env.substitute(inv.statement, &self.1);
            let mut new_deps: HashSet<_> = inv
                .dependencies
                .into_iter()
                .map(|d| env.substitute(d, &self.1))
                .collect();
            for inv in self
                .substitution_justifications(env)
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
        invs
    }
}

impl_any_eq_for_construct!(CSubstitution);

impl Construct for CSubstitution {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, env: &mut Environment<'x>, _this: ConstructId, _scope: Box<dyn Scope>) {
        if let Err(err) = self
            .substitution_justifications(env)
            .borrow()
            .as_ref()
            .unwrap()
        {
            println!("{}", err);
            todo!("nice error");
        }
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        let mut deps = Dependencies::new();
        let base = env.get_dependencies(self.0);
        for dep in base.as_variables() {
            if let Some((_, rep)) = self.1.iter().find(|(var, _)| var.is_same_variable_as(&dep)) {
                let replaced_deps = env.get_dependencies(*rep);
                for rdep in replaced_deps
                    .into_variables()
                    .skip(dep.get_dependencies().len())
                {
                    deps.push_eager(rdep);
                }
            } else {
                if let Some(subbed_var) = dep.inline_substitute(env, &self.1) {
                    deps.push_eager(subbed_var);
                }
            }
        }
        for inv in self
            .substitution_justifications(env)
            .borrow()
            .iter()
            .flatten()
            .flatten()
        {
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
    ) -> Vec<Invariant> {
        self.invariants(env)
    }

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        other: SubExpr,
    ) -> TripleBool {
        let mut new_subs = subs.clone();
        for (target, value) in &self.1 {
            new_subs.insert_or_replace(target.clone(), SubExpr(*value, subs));
        }
        env.is_def_equal(SubExpr(self.0, &new_subs), other)
    }
}
