use super::{ConstructId, Environment};
use crate::constructs::ConstructDefinition;

impl<'x> Environment<'x> {
    pub fn dereference_original(&self, con_id: ConstructId) -> ConstructId {
        if let ConstructDefinition::Other(con_id) = &self.constructs[con_id].definition {
            self.dereference_original(*con_id)
        } else {
            con_id
        }
    }

    pub fn dereference_reduced(&self, con_id: ConstructId) -> ConstructId {
        if let ConstructDefinition::Other(con_id) = &self.constructs[con_id].reduced {
            self.dereference_reduced(*con_id)
        } else {
            con_id
        }
    }

    pub fn dereference_for_vomiting(&self, con_id: ConstructId) -> ConstructId {
        if self.use_reduced_definitions_while_vomiting {
            self.dereference_reduced(con_id)
        } else {
            self.dereference_original(con_id)
        }
    }

    pub fn reduce(&mut self, con_id: ConstructId) {
        self.resolve(con_id);
        if self.constructs[con_id].reduced.is_placeholder() {
            match &self.constructs[con_id].definition {
                &ConstructDefinition::Other(id) => {
                    self.constructs[con_id].reduced = ConstructDefinition::Other(id);
                },
                ConstructDefinition::Resolved(con) => {
                    let reduced = con.dyn_clone().reduce(self);
                    self.constructs[con_id].reduced = reduced;
                }
                ConstructDefinition::Unresolved(_) => unreachable!(),
            }
        }
    }

    pub fn reduce_all(&mut self) {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            self.reduce(id);
            next_id = self.constructs.next(id);
        }
    }
}
