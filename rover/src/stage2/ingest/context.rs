use crate::{
    shared::{Definitions, ItemId},
    stage2::structure::Environment,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocalInfo {
    Plain,
    Type(ItemId),
}

pub struct Context<'e, 'd> {
    pub current_item_id: Option<ItemId>,
    pub environment: &'e mut Environment,
    pub parent_scopes: Vec<&'d Definitions>,
    pub local_info: LocalInfo,
}

impl<'e, 'd> Context<'e, 'd> {
    pub fn new(environment: &'e mut Environment) -> Self {
        Self {
            current_item_id: None,
            environment,
            parent_scopes: Vec::new(),
            local_info: LocalInfo::Plain,
        }
    }

    /// Returns a new instance of self which borrows the same environment.
    fn borrow_new<'s>(&'s mut self) -> Context<'s, 'd> {
        Context {
            current_item_id: None,
            environment: self.environment,
            parent_scopes: self.parent_scopes.clone(),
            local_info: self.local_info.clone(),
        }
    }

    /// Returns a new Context with mostly the same values, except local_info is
    /// set to Plain.
    pub fn child<'s>(&'s mut self) -> Context<'s, 'd> {
        let mut ctx = self.borrow_new();
        ctx.local_info = LocalInfo::Plain;
        ctx
    }

    pub fn with_id_scope_info(
        self,
        current_item_id: ItemId,
        extra_parent_scope: &'d Definitions,
        local_info: LocalInfo,
    ) -> Self {
        self.with_current_item_id(current_item_id)
            .with_additional_scope(extra_parent_scope)
            .with_local_info(local_info)
    }

    pub fn with_current_item_id(mut self, id: ItemId) -> Self {
        self.current_item_id = Some(id);
        self
    }

    pub fn with_additional_scope(mut self, scope: &'d Definitions) -> Self {
        self.parent_scopes.push(scope);
        self
    }

    pub fn with_local_info(mut self, info: LocalInfo) -> Self {
        self.local_info = info;
        self
    }

    pub fn get_or_create_current_id(&mut self) -> ItemId {
        if self.current_item_id.is_none() {
            self.current_item_id = Some(self.environment.next_id());
        }
        self.current_item_id.unwrap()
    }
}
