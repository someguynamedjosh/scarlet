use super::{
    substitution::Substitutions, variable::CVariable, Construct, ConstructDefinition, ConstructId,
};
use crate::{environment::Environment, impl_any_eq_for_construct, shared::TripleBool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIfThenElse {
    pub condition: ConstructId,
    pub then: ConstructId,
    pub elsee: ConstructId,
}

impl_any_eq_for_construct!(CIfThenElse);

impl Construct for CIfThenElse {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        [
            env.get_dependencies(self.condition),
            env.get_dependencies(self.then),
            env.get_dependencies(self.elsee),
        ]
        .concat()
    }

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, _other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        env.reduce(self.condition);
        let condition = env.resolve(self.condition);
        match env.is_def_equal(condition, env.get_builtin_item("true")) {
            TripleBool::True => {
                env.reduce(self.then);
                self.then.into()
            }
            TripleBool::False => {
                env.reduce(self.elsee);
                self.elsee.into()
            }
            TripleBool::Unknown => self.dyn_clone().into(),
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let condition = env.substitute(self.condition, substitutions);
        let then = env.substitute(self.then, substitutions);
        let elsee = env.substitute(self.elsee, substitutions);
        env.push_construct(
            Box::new(Self {
                condition,
                then,
                elsee,
            }),
            vec![condition, then, elsee],
        )
    }
}
