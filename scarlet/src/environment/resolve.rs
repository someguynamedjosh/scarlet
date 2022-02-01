use super::{ConstructId, Environment};
use crate::{constructs::ConstructDefinition, environment::dependencies::DepResStackFrame};

impl<'x> Environment<'x> {
    pub fn resolve_all(&mut self) {
        self.for_each_construct_returning_nothing(Self::resolve);
    }

    pub fn resolve(&mut self, con_id: ConstructId) {
        let con = &self.constructs[con_id];
        if let ConstructDefinition::Unresolved(resolvable) = &con.definition {
            self.dep_res_stack.push(DepResStackFrame(con_id));
            let resolvable = resolvable.dyn_clone();
            let scope = con.scope.dyn_clone();
            let new_def = resolvable.resolve(self, scope);
            self.constructs[con_id].definition = new_def;
            assert_eq!(self.dep_res_stack.pop(), Some(DepResStackFrame(con_id)));
        }
    }
}
