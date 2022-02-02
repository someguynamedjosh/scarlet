use std::collections::HashSet;

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
#[derive(Clone, Copy, Debug)]
pub struct SubExpr<'a>(pub ConstructId, pub &'a NestedSubstitutions<'a>);
pub type NestedSubstitutions<'a> = OrderedMap<CVariable, SubExpr<'a>>;
type Justifications = Result<Vec<Invariant>, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSubstitution(ConstructId, Substitutions, Justifications);

impl CSubstitution {
    pub fn new<'x>(base: ConstructId, subs: Substitutions, env: &mut Environment) -> Self {
        let mut sel = Self(base, subs.clone(), Ok(vec![]));
        sel.2 = sel.substitution_justifications(env);
        sel
    }

    pub fn base(&self) -> ConstructId {
        self.0
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.1
    }

    fn substitution_justifications(&self, env: &mut Environment) -> Justifications {
        let mut previous_subs = Substitutions::new();
        let mut invariants = Vec::new();
        for (target, value) in &self.1 {
            match target.can_be_assigned(*value, env, &previous_subs) {
                Ok(mut new_invs) => {
                    previous_subs.insert_no_replace(target.clone(), *value);
                    invariants.append(&mut new_invs)
                }
                Err(err) => {
                    eprintln!("{:#?}", self);
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
            for inv in self.2.iter().flatten() {
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

    fn check<'x>(&self, _env: &mut Environment<'x>, _this: ConstructId, _scope: Box<dyn Scope>) {
        if let Err(err) = &self.2 {
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
                    .skip(dep.get_substitutions().len())
                {
                    deps.push_eager(rdep);
                }
            } else {
                if let Some(subbed_var) = dep.inline_substitute(env, &self.1) {
                    deps.push_eager(subbed_var);
                }
            }
        }
        for inv in self.2.iter().flatten() {
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
        let mut new_subs = NestedSubstitutions::new();
        for (target, value) in &self.1 {
            new_subs.insert_no_replace(target.clone(), SubExpr(*value, subs));
        }
        env.is_def_equal(SubExpr(self.0, &new_subs), other)
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructDefinition<'x> {
        let base = self.0;
        let mut new_subs = self.1.clone();
        // Use `substitutions` to make new substituted values for `self.1`.
        // E.G. convert thing[ sub ] to thing[ another ] if `substitutions`
        // contains sub -> another.
        for (_, value) in &mut new_subs {
            let subbed = env.substitute(*value, substitutions);
            *value = subbed;
        }
        // Figure out what substitutions should be applied to the base.
        let mut base_subs = Substitutions::new();
        for (target, value) in substitutions {
            let mut already_present = false;
            for (existing_target, _) in &new_subs {
                // Find if we already substitute that target with something else.
                if existing_target.is_same_variable_as(target) {
                    already_present = true;
                    break;
                }
            }
            // If not, we should apply it to the base.
            if !already_present {
                base_subs.insert_no_replace(target.clone(), *value);
            }
        }
        // Substitute the base using any substitutions we don't already overwrite.
        let base = env.substitute(base, &base_subs);
        // Make a new substitution with the substituted base and substitutions.
        ConstructDefinition::Resolved(Self(base, new_subs, self.2.clone()).dyn_clone())
    }
}
