use super::{Environment, ItemId, UnresolvedItemError};
use crate::{
    constructs::{base::BoxedConstruct, downcast_construct, Construct, Item, ItemDefinition},
    scope::{LookupIdentResult, Scope},
};

impl<'x> Environment<'x> {
    pub fn get_item(&self, item_id: ItemId) -> &Item<'x> {
        &self.items[item_id]
    }

    pub fn get_item_scope(&mut self, item_id: ItemId) -> &dyn Scope {
        let item = self.get_item(item_id);
        match &item.definition {
            &ItemDefinition::Other(other) => self.get_item_scope(other),
            _ => &*self.get_item(item_id).scope,
        }
    }

    pub fn lookup_ident(&mut self, item_id: ItemId, ident: &str) -> LookupIdentResult {
        self.items[item_id]
            .scope
            .dyn_clone()
            .lookup_ident(self, ident)
    }

    pub(super) fn get_construct_definition_no_deref(
        &mut self,
        item_id: ItemId,
    ) -> Result<&BoxedConstruct, UnresolvedItemError> {
        let old_item_id = item_id;
        if let ItemDefinition::Resolved(def) = &self.items[item_id].definition {
            Ok(def)
        } else if let ItemDefinition::Unresolved(..) = &self.items[item_id].definition {
            Err(UnresolvedItemError(item_id))
        } else {
            eprintln!("{:#?}", self);
            eprintln!("{:?} -> {:?}", old_item_id, item_id);
            unreachable!()
        }
    }

    pub(super) fn get_and_downcast_construct_definition_no_deref<C: Construct>(
        &mut self,
        item_id: ItemId,
    ) -> Result<Option<&C>, UnresolvedItemError> {
        Ok(downcast_construct(
            &**self.get_construct_definition_no_deref(item_id)?,
        ))
    }

    pub fn get_item_as_construct(
        &mut self,
        item_id: ItemId,
    ) -> Result<&BoxedConstruct, UnresolvedItemError> {
        let item_id = self.dereference(item_id)?;
        if let ItemDefinition::Resolved(def) = &self.items[item_id].definition {
            Ok(def)
        } else {
            Err(UnresolvedItemError(item_id))
        }
    }

    pub fn get_and_downcast_construct_definition<C: Construct>(
        &mut self,
        item_id: ItemId,
    ) -> Result<Option<&C>, UnresolvedItemError> {
        Ok(downcast_construct(&**self.get_item_as_construct(item_id)?))
    }

    pub fn set_name(&mut self, item_id: ItemId, name: String) {
        self.items[item_id].name = Some(name);
    }
}
