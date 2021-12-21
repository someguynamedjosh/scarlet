use super::{ConstructId, Environment};
use crate::{
    constructs::{base::BoxedConstruct, AnnotatedConstruct, ConstructDefinition},
    shared::{Id, TripleBool},
};

impl<'x> Environment<'x> {
    pub fn get_construct(&self, con_id: ConstructId) -> &AnnotatedConstruct<'x> {
        &self.constructs[con_id]
    }

    pub fn get_construct_definition(&mut self, con_id: ConstructId) -> &BoxedConstruct {
        let con_id = self.resolve(con_id);
        if let &ConstructDefinition::Other(id) = &self.constructs[con_id].definition {
            self.get_construct_definition(id)
        } else if let ConstructDefinition::Resolved(def) = &self.constructs[con_id].definition {
            def
        } else {
            unreachable!()
        }
    }

    pub fn generated_invariants(&mut self, con_id: ConstructId) -> Vec<ConstructId> {
        let context = self.get_construct_definition(con_id).dyn_clone();
        context.generated_invariants(con_id, self)
    }

    pub fn has_invariant(&mut self, expression: ConstructId, context_id: ConstructId) -> bool {
        let generated_invariants = self.generated_invariants(context_id);
        for inv in generated_invariants {
            if self.is_def_equal(expression, inv) == TripleBool::True {
                return true;
            }
        }
        let scope = self.get_construct(context_id).scope.dyn_clone();
        scope.lookup_invariant(self, expression)
    }
}
