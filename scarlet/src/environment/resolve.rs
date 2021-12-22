use super::{ConstructId, Environment};
use crate::constructs::ConstructDefinition;

impl<'x> Environment<'x> {
    pub fn resolve_all(&mut self) {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            self.resolve(id);
            next_id = self.constructs.next(id);
        }
    }

    pub fn resolve(&mut self, con_id: ConstructId) {
        let con = &self.constructs[con_id];
        if let ConstructDefinition::Unresolved(resolvable) = &con.definition {
            let resolvable = resolvable.dyn_clone();
            let scope = con.scope.dyn_clone();
            let new_def = resolvable.resolve(self, scope);
            self.constructs[con_id].definition = new_def;
        }
    }
}
