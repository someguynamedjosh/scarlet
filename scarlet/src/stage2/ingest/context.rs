use crate::{
    shared::{Definitions, ItemId},
    stage2::structure::Environment,
};

pub struct Context<'e, 'd> {
    pub current_item_id: Option<ItemId>,
    pub environment: &'e mut Environment,
    pub parent_scopes: Vec<&'d Definitions>,
}

impl<'e, 'd> Context<'e, 'd> {
    pub fn new(environment: &'e mut Environment) -> Self {
        Self {
            current_item_id: None,
            environment,
            parent_scopes: Vec::new(),
        }
    }

    /// Returns a new instance of self which borrows the same environment.
    fn borrow_new<'s>(&'s mut self) -> Context<'s, 'd> {
        Context {
            current_item_id: None,
            environment: self.environment,
            parent_scopes: self.parent_scopes.clone(),
        }
    }

    /// Returns a new Context with the same values
    pub fn child<'s>(&'s mut self) -> Context<'s, 'd> {
        self.borrow_new()
    }

    pub fn with_id_and_scope(
        self,
        current_item_id: ItemId,
        extra_parent_scope: &'d Definitions,
    ) -> Self {
        self.with_current_item_id(current_item_id)
            .with_additional_scope(extra_parent_scope)
    }

    pub fn with_current_item_id(mut self, id: ItemId) -> Self {
        self.current_item_id = Some(id);
        self
    }

    pub fn with_additional_scope(mut self, scope: &'d Definitions) -> Self {
        self.parent_scopes.push(scope);
        self
    }

    pub fn get_or_create_current_id(&mut self) -> ItemId {
        if self.current_item_id.is_none() {
            self.current_item_id = Some(self.environment.next_id());
        }
        self.current_item_id.unwrap()
    }
}
