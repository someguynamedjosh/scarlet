use itertools::Itertools;

use super::{variable::CVariable, Construct, ConstructId};
use crate::{
    constructs::variable::VarType, environment::Environment, impl_any_eq_for_construct,
    shared::OrderedMap,
};

pub type Substitutions = OrderedMap<CVariable, ConstructId>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSubstitution(pub ConstructId, pub Substitutions);

impl_any_eq_for_construct!(CSubstitution);

impl Construct for CSubstitution {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, env: &mut Environment<'x>) {
        for (target, value) in &self.1 {
            if !env
                .var_type_matches_var_type(&VarType::Just(*value), &target.typee)
                .is_guaranteed_match()
            {
                panic!(
                    "Argument {:?} does not match {:?}, which it is assigned to.",
                    value, target.typee
                )
            }
        }
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        env.get_dependencies(self.0)
            .into_iter()
            .map(|var| var.substitute(env, &self.1))
            .collect_vec()
            .into_iter()
            .map(|item| env.get_dependencies(item))
            .flatten()
            .collect()
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        env.substitute(self.0, &self.1)
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let mut new_subs = self.1.clone();
        for (_, value) in &mut new_subs {
            let subbed = env.substitute(*value, substitutions);
            *value = subbed;
        }
        for (target, value) in substitutions {
            let mut already_present = false;
            for (existing_target, _) in &new_subs {
                if existing_target.is_same_variable_as(target) {
                    already_present = true;
                    break;
                }
            }
            if !already_present {
                new_subs.insert_no_replace(target.clone(), *value);
            }
        }
        env.push_construct(Box::new(Self(self.0, new_subs)))
    }
}
