use super::{
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{environment::Environment, impl_any_eq_for_construct};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Member {
    Named(String),
    Index {
        index: ConstructId,
        proof_lt_len: ConstructId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CMember(pub ConstructId, pub Member);

impl_any_eq_for_construct!(CMember);

impl Construct for CMember {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, env: &mut Environment<'x>) {
        todo!()
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = env.get_dependencies(self.0);
        if let &Member::Index { index, proof_lt_len } = &self.1 {
            deps.append(&mut env.get_dependencies(index));
            deps.append(&mut env.get_dependencies(proof_lt_len));
        }
        deps
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let base = env.reduce(self.0);
        let member = match &self.1 {
            Member::Named(..) => self.1.clone(),
            Member::Index {
                index,
                proof_lt_len,
            } => Member::Index {
                index: env.reduce(*index),
                proof_lt_len: env.reduce(*proof_lt_len),
            },
        };
        env.push_construct(Box::new(Self(base, member)))
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let base = env.substitute(self.0, substitutions);
        let member = match &self.1 {
            &Member::Named(..) => self.1.clone(),
            &Member::Index {
                index,
                proof_lt_len,
            } => Member::Index {
                index: env.substitute(index, substitutions),
                proof_lt_len: env.substitute(proof_lt_len, substitutions),
            },
        };
        env.push_construct(Box::new(Self(base, member)))
    }
}
