use super::{Environment, ItemId, UnresolvedItemError};
use crate::constructs::{
    structt::{AtomicStructMember, CAtomicStructMember, CPopulatedStruct},
    ItemDefinition,
};

impl<'x> Environment<'x> {
    pub fn dereference(&mut self, item_id: ItemId) -> Result<ItemId, UnresolvedItemError> {
        if let &ItemDefinition::Other(item_id) = &self.items[item_id].definition {
            return self.dereference(item_id);
        } else if let Some(mem) =
            self.get_and_downcast_construct_definition_no_deref::<CAtomicStructMember>(item_id)?
        {
            let mem = mem.clone();
            if let Some(structt) =
                self.get_and_downcast_construct_definition::<CPopulatedStruct>(mem.0)?
            {
                let id = match mem.1 {
                    AtomicStructMember::Label => todo!(),
                    AtomicStructMember::Value => structt.get_value(),
                    AtomicStructMember::Rest => structt.get_rest(),
                };
                return self.dereference(id);
            }
        }
        Ok(item_id)
    }
    
    pub fn dereference_no_unresolved_error(&mut self, item_id: ItemId) -> ItemId {
        if let &ItemDefinition::Other(item_id) = &self.items[item_id].definition {
            return self.dereference_no_unresolved_error(item_id);
        } else if let Ok(Some(mem)) =
            self.get_and_downcast_construct_definition_no_deref::<CAtomicStructMember>(item_id)
        {
            let mem = mem.clone();
            if let Ok(Some(structt)) =
                self.get_and_downcast_construct_definition::<CPopulatedStruct>(mem.0)
            {
                let id = match mem.1 {
                    AtomicStructMember::Label => todo!(),
                    AtomicStructMember::Value => structt.get_value(),
                    AtomicStructMember::Rest => structt.get_rest(),
                };
                return self.dereference_no_unresolved_error(id);
            }
        }
        item_id
    }
}
