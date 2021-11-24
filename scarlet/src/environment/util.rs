use std::convert::TryInto;

use super::{ConstructId, Environment};
use crate::constructs::{as_builtin_value, base::BoxedConstruct, builtin_value::CBuiltinValue};

impl<'x> Environment<'x> {
    pub fn get_construct(&mut self, con_id: ConstructId) -> &BoxedConstruct {
        let con_id = self.resolve(con_id);
        self.constructs[con_id].definition.as_resolved().unwrap()
    }

    pub fn get_builtin_value<T>(&mut self, con_id: ConstructId) -> Option<T>
    where
        CBuiltinValue: TryInto<T>,
    {
        let con = self.get_construct(con_id);
        if let Some(bv) = as_builtin_value(&**con) {
            <CBuiltinValue as TryInto<T>>::try_into(*bv).ok()
        } else {
            None
        }
    }
}
