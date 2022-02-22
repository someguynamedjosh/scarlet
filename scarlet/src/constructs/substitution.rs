use super::{
    variable::{CVariable, VariableId},
    Construct, ConstructId, GenInvResult,
};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
        invariants::Invariant,
        sub_expr::{NestedSubstitutions, SubExpr},
        CheckResult, Environment,
    },
    impl_any_eq_for_construct,
    scope::Scope,
    shared::OrderedMap,
};

pub type Substitutions = OrderedMap<VariableId, ConstructId>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSubstitution {
    base: ConstructId,
    subs: Substitutions,
    invs: Vec<crate::environment::invariants::Invariant>,
}

impl CSubstitution {
    pub fn new<'x>(
        base: ConstructId,
        subs: Substitutions,
        invs: Vec<crate::environment::invariants::Invariant>,
    ) -> Self {
        Self { base, subs, invs }
    }

    pub fn new_unchecked(base: ConstructId, subs: Substitutions) -> Self {
        Self::new(base, subs, Vec::new())
    }

    pub fn base(&self) -> ConstructId {
        self.base
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.subs
    }
}

impl_any_eq_for_construct!(CSubstitution);

impl Construct for CSubstitution {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(
        &self,
        _env: &mut Environment<'x>,
        _this: ConstructId,
        _scope: Box<dyn Scope>,
    ) -> CheckResult {
        Ok(())
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let mut deps = Dependencies::new();
        let base = env.get_dependencies(self.base);
        let base_error = base.error();
        for dep in base.as_variables() {
            if let Some((_, rep)) = self.subs.iter().find(|(var, _)| *var == dep.id) {
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
        for inv in self.invs.iter() {
            for &dep in &inv.dependencies {
                if let Ok(Some(var)) = env.get_and_downcast_construct_definition::<CVariable>(dep) {
                    let id = var.get_id();
                    deps.push_eager(env.get_variable(id).clone().as_dependency(env));
                } else {
                    deps.append(env.get_dependencies(dep));
                }
            }
        }
        deps
    }

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        _env: &mut Environment<'x>,
    ) -> GenInvResult {
        self.invs.clone()
    }

    fn deq_priority<'x>(&self) -> DeqPriority {
        6
    }

    fn discover_equality<'x>(
        &self,
        env: &mut Environment<'x>,
        other_id: ConstructId,
        other: &dyn Construct,
        limit: u32,
        tiebreaker: DeqSide,
    ) -> DeqResult {
        let base = env.discover_equal_with_tiebreaker(self.base, other_id, limit, tiebreaker)?;
        let mut result = base.clone().without_subs();
        if let Equal::Yes(left, right) = base {
            for (target, proposed_value) in &self.subs {
                if let Some(existing_value) = left.get(&target) {
                    let proposed_equals_existing = env.discover_equal_with_tiebreaker(
                        *proposed_value,
                        *existing_value,
                        limit,
                        tiebreaker,
                    )?;
                    result = Equal::and(vec![result, proposed_equals_existing]);
                } else {
                    if other.get_dependencies(env).contains_var(*target) {
                        let extra_sub: Substitutions =
                            vec![(*target, *proposed_value)].into_iter().collect();
                        result =
                            Equal::and(vec![result, Equal::Yes(Default::default(), extra_sub)]);
                    }
                }
            }
            for (target, value) in right {
                let mut value_subs = Substitutions::new();
                for dep in env.get_dependencies(value).into_variables() {
                    if left.contains_key(&dep.id) {
                        continue;
                    }
                    if let Some(&rep) = self.subs.get(&dep.id) {
                        value_subs.insert_no_replace(dep.id, rep);
                    }
                }
                let mut new_value = env.substitute(value, &value_subs);

                // Special handling of things like x[x IS y] and u[u IS v]
                if value_subs.len() == 1 {
                    let &(value_sub_target, value_sub_value) = value_subs.iter().next().unwrap();
                    let value_sub_target = env.get_variable(value_sub_target);
                    if value == value_sub_target.construct.unwrap() {
                        new_value = value_sub_value;
                    }
                }

                // Special handling of substitutions like x IS x and u IS u.
                let target_con = env.get_variable(target).construct.unwrap();
                if target_con == new_value {
                    continue;
                }

                println!("{:?} is {:?} sub {:?}", new_value, value, &value_subs);
                let this_sub = vec![(target, new_value)].into_iter().collect();
                result = Equal::and(vec![result, Equal::Yes(Default::default(), this_sub)])
            }
            for (target, value) in left {
                if self.subs.contains_key(&target) {
                    continue;
                }
                let this_sub = vec![(target, value)].into_iter().collect();
                result = Equal::and(vec![result, Equal::Yes(this_sub, Default::default())]);
            }
        }
        Ok(result)
    }
}
