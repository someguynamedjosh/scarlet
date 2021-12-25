use super::{variable::CVariable, Construct, ConstructDefinition, ConstructId};
use crate::{
    environment::Environment,
    impl_any_eq_for_construct,
    scope::Scope,
    shared::{OrderedMap, TripleBool},
};

pub type Substitutions = OrderedMap<CVariable, ConstructId>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSubstitution(ConstructId, Substitutions);

impl CSubstitution {
    pub fn new<'x>(base: ConstructId, subs: Substitutions) -> Self {
        Self(base, subs.clone())
    }
}

impl_any_eq_for_construct!(CSubstitution);

impl Construct for CSubstitution {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, env: &mut Environment<'x>) {
        for (target, value) in &self.1 {
            if !target.can_be_assigned(*value, env) {
                println!("{:#?}", env);
                todo!(
                    "nice error, argument {:?} does not meet all of {:?}'s invariants",
                    value,
                    target
                );
            }
        }
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = Vec::new();
        for dep in env.get_dependencies(self.0) {
            if let Some(rep) = self.1.get(&dep) {
                deps.append(&mut env.get_dependencies(*rep));
            } else {
                deps.push(dep);
            }
        }
        deps
    }

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, _other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        self.check(env);
        let subbed = env.substitute(self.0, &self.1);
        env.reduce(subbed);
        subbed.into()
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
        scope: Box<dyn Scope>,
    ) -> ConstructId {
        let base = self.0;
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
        env.push_construct(Self::new(base, new_subs), scope)
    }
}
