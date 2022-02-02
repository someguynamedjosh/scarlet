use super::{ConstructId, Environment};
use crate::constructs::{
    structt::{AtomicStructMember, CAtomicStructMember, CPopulatedStruct},
    ConstructDefinition,
};

impl<'x> Environment<'x> {
    pub fn dereference(&mut self, con_id: ConstructId) -> ConstructId {
        if let &ConstructDefinition::Other(con_id) = &self.constructs[con_id].definition {
            self.resolve(con_id);
            return self.dereference(con_id);
        } else if let Some(mem) =
            self.get_and_downcast_construct_definition_no_deref::<CAtomicStructMember>(con_id)
        {
            let mem = mem.clone();
            if let Some(structt) =
                self.get_and_downcast_construct_definition::<CPopulatedStruct>(mem.0)
            {
                let id = match mem.1 {
                    AtomicStructMember::Label => todo!(),
                    AtomicStructMember::Value => structt.get_value(),
                    AtomicStructMember::Rest => structt.get_rest(),
                };
                self.resolve(id);
                return self.dereference(id);
            }
        }
        con_id
    }
}
