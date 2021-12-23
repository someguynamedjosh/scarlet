use crate::constructs::{Construct, ConstructDefinition};

use super::{ConstructId, Environment};

impl<'x> Environment<'x> {
    pub fn dereference(&self, con_id: ConstructId) -> ConstructId {
        if let ConstructDefinition::Other(con_id) = &self.constructs[con_id].definition {
            self.dereference(*con_id)
        } else {
            con_id
        }
    }

    pub fn reduce(&mut self, con_id: ConstructId) {
        let con_id = self.dereference(con_id);
        let con = self.get_construct_definition(con_id).dyn_clone();
        let reduced = con.reduce(self);
        self.constructs[con_id].definition = reduced;
    }

    pub fn reduce_all(&mut self) {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            self.reduce(id);
            next_id = self.constructs.next(id);
        }
    }
}
