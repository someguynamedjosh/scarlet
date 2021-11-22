use super::{
    as_struct,
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{environment::Environment, impl_any_eq_for_construct};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CMember(pub ConstructId, pub String);

impl_any_eq_for_construct!(CMember);

impl Construct for CMember {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        env.get_dependencies(self.0)
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let base = env.reduce(self.0);
        match &self.1 {
            name => {
                if let Some(structt) = as_struct(&**env.get_construct(base)) {
                    for (index, field) in structt.0.iter().enumerate() {
                        if field.name.as_ref().map(|n| n == name).unwrap_or(false)
                            || name == &format!("{}", index)
                        {
                            return field.value;
                        }
                    }
                }
            }
        }
        env.push_construct(Box::new(Self(base, self.1.clone())))
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let base = env.substitute(self.0, substitutions);
        let member = self.1.clone();
        env.push_construct(Box::new(Self(base, member)))
    }
}
