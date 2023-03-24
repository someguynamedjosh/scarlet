use crate::{
    diagnostic::Diagnostic,
    environment::{Environment, ItemId},
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
}
