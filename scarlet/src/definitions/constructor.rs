use super::builtin::DBuiltin;
use crate::{
    diagnostic::Diagnostic,
    environment::{Env, ItemId},
};

#[derive(Clone, Debug)]
pub struct DConstructor {
    r#type: ItemId,
}

impl DConstructor {
    pub fn new(r#type: ItemId) -> Self {
        Self { r#type }
    }

    pub fn r#type(&self) -> ItemId {
        self.r#type
    }

    pub fn add_type_asserts(&self, env: &mut Env) {
        let god_type = env.god_type();
        env.assert_of_type(self.r#type, god_type);
    }
}
