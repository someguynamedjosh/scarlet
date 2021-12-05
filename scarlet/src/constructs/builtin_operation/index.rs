use super::BuiltinOperation;
use crate::{
    constructs::{as_builtin_value, as_struct, builtin_value::CBuiltinValue, ConstructId},
    environment::Environment,
    impl_any_eq_for_construct,
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OIndex;

impl_any_eq_for_construct!(OIndex);

impl BuiltinOperation for OIndex {
    fn check<'x>(&self, env: &mut Environment<'x>, args: &[ConstructId]) {
        todo!()
    }

    fn compute<'x>(&self, env: &mut Environment<'x>, args: &[ConstructId]) -> Option<ConstructId> {
        let base = args[0];
        let index = args[1];
        if let Some(&CBuiltinValue::_32U(index)) = as_builtin_value(&**env.get_construct(index)) {
            if let Some(structt) = as_struct(&**env.get_construct(base)) {
                return Some(structt.0[index as usize].value);
            }
        }
        None
    }

    fn dyn_clone(&self) -> Box<dyn BuiltinOperation> {
        Box::new(self.clone())
    }
}
