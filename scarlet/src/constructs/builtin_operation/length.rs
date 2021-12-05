use super::BuiltinOperation;
use crate::{
    constructs::{as_struct, builtin_value::CBuiltinValue, ConstructId},
    environment::Environment,
    impl_any_eq_for_construct,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OLength;

impl_any_eq_for_construct!(OLength);

impl BuiltinOperation for OLength {
    fn check<'x>(&self, env: &mut Environment<'x>, args: &[ConstructId]) {
        todo!()
    }

    fn compute<'x>(&self, env: &mut Environment<'x>, args: &[ConstructId]) -> Option<ConstructId> {
        let base = env.reduce(args[0]);
        let base_con = env.get_construct(base);
        if let Some(structt) = as_struct(&**base_con) {
            let length = structt.0.len() as u32;
            Some(env.push_construct(Box::new(CBuiltinValue::_32U(length))))
        } else {
            None
        }
    }

    fn dyn_clone(&self) -> Box<dyn BuiltinOperation> {
        Box::new(self.clone())
    }
}
