use super::{
    base::{Construct, ConstructId},
    builtin_operation::{BuiltinOperation, CBuiltinOperation},
    substitution::Substitutions,
    variable::{CVariable, VarType},
};
use crate::{
    constructs::{builtin_value::CBuiltinValue, length::CLength},
    environment::Environment,
    impl_any_eq_for_construct,
};

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
        if let &Member::Index {
            index,
            proof_lt_len,
        } = &self.1
        {
            let len = env.push_construct(Box::new(CLength(self.0)));
            let lt_len = env.push_construct(Box::new(CBuiltinOperation {
                op: BuiltinOperation::LessThan32U,
                args: vec![index, len],
            }));
            let truee = env.push_construct(Box::new(CBuiltinValue::Bool(true)));
            let lt_len_and_true = VarType::And(lt_len, truee).reduce(env);
            if !env
                .var_type_matches_var_type(&VarType::Just(proof_lt_len), &lt_len_and_true)
                .is_guaranteed_match()
            {
                todo!(
                    "Nice error, {:?} is not a proof that {:?} is in bounds of {:?}",
                    proof_lt_len,
                    index,
                    self.0
                )
            }
        }
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = env.get_dependencies(self.0);
        if let &Member::Index {
            index,
            proof_lt_len,
        } = &self.1
        {
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
