use super::{
    as_builtin_value, as_struct,
    base::{Construct, ConstructId},
    builtin_operation::{BuiltinOperation, CBuiltinOperation},
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{
    constructs::{builtin_value::CBuiltinValue, length::CLength},
    environment::Environment,
    impl_any_eq_for_construct,
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIndex {
    pub base: ConstructId,
    pub index: ConstructId,
    pub proof_lt_len: ConstructId,
}

impl_any_eq_for_construct!(CIndex);

impl Construct for CIndex {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, env: &mut Environment<'x>) {
        todo!("check proof_lt_len matches lt_len and true or something like that");
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = env.get_dependencies(self.base);
        deps.append(&mut env.get_dependencies(self.index));
        deps.append(&mut env.get_dependencies(self.proof_lt_len));
        deps
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let base = env.reduce(self.base);
        let index = env.reduce(self.index);
        let proof_lt_len = env.reduce(self.proof_lt_len);
        if let Some(&CBuiltinValue::_32U(index)) = as_builtin_value(&**env.get_construct(index)) {
            if let Some(&CBuiltinValue::Bool(true)) =
                as_builtin_value(&**env.get_construct(proof_lt_len))
            {
                if let Some(structt) = as_struct(&**env.get_construct(base)) {
                    return structt.0[index as usize].value;
                }
            }
        }
        env.push_construct(Box::new(Self {
            base,
            index,
            proof_lt_len,
        }))
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let base = env.substitute(self.base, substitutions);
        let index = env.substitute(self.index, substitutions);
        let proof_lt_len = env.substitute(self.proof_lt_len, substitutions);
        env.push_construct(Box::new(Self {
            base,
            index,
            proof_lt_len,
        }))
    }
}
