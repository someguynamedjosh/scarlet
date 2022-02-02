use super::{ConstructId, Environment};
use crate::{
    constructs::{substitution::Substitutions, ConstructDefinition},
    shared::TripleBool,
};

#[derive(Debug)]
pub struct SubstituteStackFrame {
    base: ConstructId,
    substitutions: Substitutions,
    into: ConstructId,
}
pub type SubstituteStack = Vec<SubstituteStackFrame>;

impl<'x> Environment<'x> {
    pub fn substitute(
        &mut self,
        con_id: ConstructId,
        substitutions: &Substitutions,
    ) -> ConstructId {
        if substitutions.len() == 0 {
            return con_id;
        }

        for frame in &self.substitute_stack {
            if frame.base == con_id && &frame.substitutions == substitutions {
                return frame.into;
            }
        }

        let scope = self.get_construct(con_id).scope.dyn_clone();
        let into = self.push_placeholder(scope);
        self.substitute_stack.push(SubstituteStackFrame {
            base: con_id,
            substitutions: substitutions.clone(),
            into,
        });

        let def = self.get_construct_definition(con_id).dyn_clone();
        let subbed = def.substitute(self, substitutions);
        let frame = self.substitute_stack.pop().unwrap();
        assert_eq!(frame.into, into);
        if let ConstructDefinition::Resolved(subbed) = subbed {
            if def.is_def_equal(self, &*subbed) == TripleBool::True {
                self.constructs[into].definition = ConstructDefinition::Other(con_id);
                con_id
            } else {
                self.constructs[into].definition = ConstructDefinition::Resolved(subbed);
                into
            }
        } else {
            self.constructs[into].definition = subbed;
            into
        }
    }
}
