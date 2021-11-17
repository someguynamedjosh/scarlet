use super::{ConstructDefinition, ConstructId, Environment};
use crate::tokens::structure::Token;

impl<'x> Environment<'x> {
    pub fn reduce(&mut self, con_id: ConstructId) -> ConstructId {
        let con_id = self.resolve(con_id);
        let con = self.constructs[con_id]
            .definition
            .as_resolved()
            .unwrap()
            .dyn_clone();
        con.check(self);
        con.reduce(self, con_id)
    }
}
