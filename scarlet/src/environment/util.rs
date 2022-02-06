use super::{dependencies::DepResStackFrame, ConstructId, Environment};
use crate::{
    constructs::{
        base::BoxedConstruct, downcast_construct, substitution::SubExpr, AnnotatedConstruct,
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

    pub fn get_construct_scope(&mut self, con_id: ConstructId) -> &dyn Scope {
        self.resolve(con_id);
        let con = self.get_construct(con_id);
        match &con.definition {
            ConstructDefinition::Other(other) => self.get_construct_scope(*other),
            ConstructDefinition::Resolved(_) => &*self.get_construct(con_id).scope,
            ConstructDefinition::Unresolved(_) => unreachable!(),
        }
    }

    pub(super) fn get_construct_definition_no_deref(
        &mut self,
        con_id: ConstructId,
    ) -> &BoxedConstruct {
        let old_con_id = con_id;
        self.resolve(con_id);
        if let ConstructDefinition::Resolved(def) = &self.constructs[con_id].definition {
            def
        } else {
            println!("{:#?}", self);
            println!("{:?} -> {:?}", old_con_id, con_id);
            unreachable!()
        }
    }

    pub(super) fn get_and_downcast_construct_definition_no_deref<C: Construct>(
        &mut self,
        con_id: ConstructId,
    ) -> Option<&C> {
        downcast_construct(&**self.get_construct_definition_no_deref(con_id))
    }

    pub fn get_construct_definition(&mut self, con_id: ConstructId) -> &BoxedConstruct {
        let old_con_id = con_id;
        self.resolve(con_id);
        let con_id = self.dereference(con_id);
        if let ConstructDefinition::Resolved(def) = &self.constructs[con_id].definition {
            def
        } else {
            println!("{:#?}", self);
            println!("{:?} -> {:?}", old_con_id, con_id);
            unreachable!()
        }
    }

    pub fn get_and_downcast_construct_definition<C: Construct>(
        &mut self,
        con_id: ConstructId,
    ) -> Option<&C> {
        downcast_construct(&**self.get_construct_definition(con_id))
    }

    pub fn generated_invariants(&mut self, con_id: ConstructId) -> Vec<Invariant> {
        for frame in &self.dep_res_stack {
            if frame.0 == con_id {
                return Vec::new();
            }
        }
        self.dep_res_stack.push(DepResStackFrame(con_id));

        // if let Some(invariants) = &self.constructs[con_id].invariants {
        //     let result = invariants.clone();
        //     self.dep_res_stack.pop();
        //     result
        // } else {
        let context = self.get_construct_definition(con_id).dyn_clone();
        let invs = context.generated_invariants(con_id, self);
        self.constructs[con_id].invariants = Some(invs.clone());
        self.dep_res_stack.pop();
        invs
        // }
    }

    pub fn get_produced_invariant(
        &mut self,
        statement: ConstructId,
        context_id: ConstructId,
        limit: u32,
    ) -> Option<Invariant> {
        let generated_invariants = self.generated_invariants(context_id);
        for inv in generated_invariants {
            if self.is_def_equal(
                SubExpr(statement, &Default::default()),
                SubExpr(inv.statement, &Default::default()),
                limit,
            ) == TripleBool::True
            {
                return Some(inv);
            }
        }
        let scope = self.get_construct(context_id).scope.dyn_clone();
        let inv = scope.lookup_invariant(self, statement);
        inv
    }
}
