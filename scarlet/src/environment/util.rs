use super::{ConstructId, Environment};
use crate::{
    constructs::{
        base::BoxedConstruct, downcast_boxed_construct, downcast_construct, AnnotatedConstruct,
        Construct, ConstructDefinition, Invariant,
    },
    scope::Scope,
    shared::TripleBool,
};

#[derive(Debug)]
pub struct InvariantStackFrame {
    con_id: ConstructId,
}
pub type InvariantStack = Vec<InvariantStackFrame>;

impl<'x> Environment<'x> {
    pub fn get_construct(&self, con_id: ConstructId) -> &AnnotatedConstruct<'x> {
        &self.constructs[con_id]
    }

    pub fn get_original_construct_scope(&mut self, con_id: ConstructId) -> &dyn Scope {
        self.resolve(con_id);
        self.reduce(con_id);
        let con = self.get_construct(con_id);
        match &con.definition {
            ConstructDefinition::Other(other) => self.get_original_construct_scope(*other),
            ConstructDefinition::Resolved(_) => &*self.get_construct(con_id).scope,
            ConstructDefinition::Unresolved(_) => unreachable!(),
        }
    }

    pub fn get_reduced_construct_definition(&mut self, con_id: ConstructId) -> &BoxedConstruct {
        let old_con_id = con_id;
        self.reduce(con_id);
        if let &ConstructDefinition::Other(id) = &self.constructs[con_id].reduced {
            self.get_reduced_construct_definition(id)
        } else if let ConstructDefinition::Resolved(def) = &self.constructs[con_id].reduced {
            def
        } else {
            println!("{:#?}", self);
            println!("{:?} -> {:?}", old_con_id, con_id);
            unreachable!()
        }
    }

    pub fn get_original_construct_definition(&mut self, con_id: ConstructId) -> &BoxedConstruct {
        let old_con_id = con_id;
        self.resolve(con_id);
        if let &ConstructDefinition::Other(id) = &self.constructs[con_id].definition {
            self.get_original_construct_definition(id)
        } else if let ConstructDefinition::Resolved(def) = &self.constructs[con_id].definition {
            def
        } else {
            println!("{:#?}", self);
            println!("{:?} -> {:?}", old_con_id, con_id);
            unreachable!()
        }
    }

    pub fn get_construct_definition_for_vomiting<CastTo: Construct>(
        &mut self,
        con_id: ConstructId,
    ) -> Option<CastTo> {
        let def = if self.use_reduced_definitions_while_vomiting {
            self.get_reduced_construct_definition(con_id)
        } else {
            self.get_original_construct_definition(con_id)
        };
        downcast_boxed_construct(def.dyn_clone())
    }

    pub fn generated_invariants(&mut self, con_id: ConstructId) -> Vec<Invariant> {
        for frame in &self.invariant_stack {
            if frame.con_id == con_id {
                return Vec::new();
            }
        }
        self.invariant_stack.push(InvariantStackFrame { con_id });

        if let Some(invariants) = &self.constructs[con_id].invariants {
            let result = invariants.clone();
            self.invariant_stack.pop();
            result
        } else {
            let context = self.get_original_construct_definition(con_id).dyn_clone();
            let invs = context.generated_invariants(con_id, self);
            self.constructs[con_id].invariants = Some(invs.clone());
            self.invariant_stack.pop();
            invs
        }
    }

    pub fn get_produced_invariant(
        &mut self,
        statement: ConstructId,
        context_id: ConstructId,
    ) -> Option<Invariant> {
        let generated_invariants = self.generated_invariants(context_id);
        for inv in generated_invariants {
            if self.is_def_equal(statement, inv.statement) == TripleBool::True {
                return Some(inv);
            }
        }
        let scope = self.get_construct(context_id).scope.dyn_clone();
        let inv = scope.lookup_invariant(self, statement);
        inv
    }
}
