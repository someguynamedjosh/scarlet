use super::{
    as_builtin_value, as_struct,
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
        let (index, proof_lt_len) = (env.reduce(self.index), env.reduce(self.proof_lt_len));
        let len = env.push_construct(Box::new(CLength(self.base)));
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
            println!("{:#?}", env);
            println!("{:?}", lt_len_and_true);
            todo!(
                "Nice error, {:?} is not a proof that {:?} is in bounds of {:?}",
                proof_lt_len,
                index,
                self.base
            )
        }
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = env.get_dependencies(self.base);
        deps.append(&mut env.get_dependencies(self.index));
        deps.append(&mut env.get_dependencies(self.proof_lt_len));
        deps
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
