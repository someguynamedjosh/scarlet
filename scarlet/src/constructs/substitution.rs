use itertools::Itertools;

use super::{variable::CVariable, Construct, ConstructDefinition, ConstructId};
use crate::{
    environment::Environment,
    impl_any_eq_for_construct,
    shared::{OrderedMap, TripleBool},
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
            match target.can_be_assigned(*value, env) {
                TripleBool::True => (),
                TripleBool::False => todo!("nice error, argument {:?} definitely cannot be assigned to {:?}", value, target),
                TripleBool::Unknown => todo!("nice error, argument {:?} might not be assignable to {:?}", value, target),
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

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, _other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        let subbed = env.substitute(self.0, &self.1);
        env.reduce(subbed);
        subbed.into()
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let base = self.0;
        let mut new_subs = self.1.clone();
        let mut contains = vec![base];
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
                contains.push(*value);
                new_subs.insert_no_replace(target.clone(), *value);
            }
        }
        env.push_construct(Box::new(Self(base, new_subs)), contains)
    }
}
