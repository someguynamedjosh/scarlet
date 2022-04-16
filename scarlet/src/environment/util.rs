use super::{Environment, ItemPtr, UnresolvedItemError};
use crate::{
    item::{downcast_construct, ItemDefinition, Item, },
    scope::{LookupIdentResult, Scope},
};

impl Environment {
    pub fn get_item(&self, item_id: ItemPtr) -> &Item {
        &self.items[item_id]
    }

    pub fn get_item_scope(&mut self, item_id: ItemPtr) -> &dyn Scope {
        let item = self.get_item(item_id);
        match &item.definition {
            &ItemDefinition::Other(other) => self.get_item_scope(other),
            _ => &*self.get_item(item_id).scope,
        }
    }

    pub fn lookup_ident(&mut self, item_id: ItemPtr, ident: &str) -> LookupIdentResult {
        self.items[item_id]
            .scope
            .dyn_clone()
            .lookup_ident(self, ident)
    }

    pub(super) fn get_construct_definition_no_deref(
        &mut self,
        item_id: ItemPtr,
    ) -> Result<&BoxedConstruct, UnresolvedItemError> {
        let old_item_id = item_id;
        if let ItemDefinition::Resolved(def) = &self.items[item_id].definition {
            Ok(def)
        } else if let ItemDefinition::Placeholder = &self.items[item_id].definition {
            Err(UnresolvedItemError(item_id))
        } else {
            eprintln!("{:#?}", self);
            eprintln!("{:?} -> {:?}", old_item_id, item_id);
            unreachable!()
        }
    }

    pub(super) fn get_and_downcast_construct_definition_no_deref<C: ItemDefinition>(
        &mut self,
        item_id: ItemPtr,
    ) -> Result<Option<&C>, UnresolvedItemError> {
        Ok(downcast_construct(
            &**self.get_construct_definition_no_deref(item_id)?,
        ))
    }

    pub fn get_item_as_construct(
        &mut self,
        item_id: ItemPtr,
    ) -> Result<&BoxedConstruct, UnresolvedItemError> {
        let item_id = self.dereference(item_id)?;
        if let ItemDefinition::Resolved(def) = &self.items[item_id].definition {
            Ok(def)
        } else {
            Err(UnresolvedItemError(item_id))
        }
    }

    pub fn get_and_downcast_construct_definition<C: ItemDefinition>(
        &mut self,
        item_id: ItemPtr,
    ) -> Result<Option<&C>, UnresolvedItemError> {
        Ok(downcast_construct(&**self.get_item_as_construct(item_id)?))
    }

    pub fn set_name(&mut self, item_id: ItemPtr, name: String) {
        self.items[item_id].name = Some(name);
    }

    pub fn set_scope(&mut self, item_id: ItemPtr, scope: Box<dyn Scope>) {
        self.items[item_id].scope = scope;
    }

    pub fn item_is_or_contains_item(
        &mut self,
        original: ItemPtr,
        is_or_contains: ItemPtr,
    ) -> Result<bool, UnresolvedItemError> {
        Ok(if original == is_or_contains {
            true
        } else {
            for content in self.get_item_as_construct(original)?.contents().clone() {
                if self.item_is_or_contains_item(content, is_or_contains)? {
                    return Ok(true);
                }
            }
            false
        })
    }
}
