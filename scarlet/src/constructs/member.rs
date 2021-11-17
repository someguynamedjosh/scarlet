use super::base::{Construct, ConstructId};
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
}
