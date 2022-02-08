use super::{dependencies::DepResStackFrame, ConstructId, Environment, UnresolvedConstructError};
use crate::{
    constructs::{
        base::BoxedConstruct, downcast_construct, substitution::SubExpr, AnnotatedConstruct,
        Construct, ConstructDefinition, GenInvResult, Invariant,
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
    ) -> Result<&BoxedConstruct, UnresolvedConstructError> {
        let old_con_id = con_id;
        if let ConstructDefinition::Resolved(def) = &self.constructs[con_id].definition {
            Ok(def)
        } else if let ConstructDefinition::Unresolved(..) = &self.constructs[con_id].definition {
            Err(UnresolvedConstructError(con_id))
        } else {
            eprintln!("{:#?}", self);
            eprintln!("{:?} -> {:?}", old_con_id, con_id);
            unreachable!()
        }
    }

    pub(super) fn get_and_downcast_construct_definition_no_deref<C: Construct>(
        &mut self,
        con_id: ConstructId,
    ) -> Result<Option<&C>, UnresolvedConstructError> {
        Ok(downcast_construct(&**self.get_construct_definition_no_deref(con_id)?))
    }

    pub fn get_construct_definition(
        &mut self,
        con_id: ConstructId,
    ) -> Result<&BoxedConstruct, UnresolvedConstructError> {
        let old_con_id = con_id;
        let con_id = self.dereference(con_id)?;
        if let ConstructDefinition::Resolved(def) = &self.constructs[con_id].definition {
            Ok(def)
        } else {
            Err(UnresolvedConstructError(con_id))
        }
    }

    pub fn get_and_downcast_construct_definition<C: Construct>(
        &mut self,
        con_id: ConstructId,
    ) -> Result<Option<&C>, UnresolvedConstructError> {
        Ok(downcast_construct(
            &**self.get_construct_definition(con_id)?,
        ))
    }

    pub fn generated_invariants(&mut self, con_id: ConstructId) -> GenInvResult {
        for frame in &self.dep_res_stack {
            if frame.0 == con_id {
                return Ok(Vec::new());
            }
        }

        let def = &self.constructs[con_id].definition;
        if def.as_other().is_some() || def.as_resolved().is_some() {
            self.dep_res_stack.push(DepResStackFrame(con_id));
            let context = self.get_construct_definition(con_id)?;
            let context = context.dyn_clone();
            let invs = context.generated_invariants(con_id, self)?;
            self.constructs[con_id].invariants = Some(invs.clone());
            self.dep_res_stack.pop();
            Ok(invs)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn get_produced_invariant(
        &mut self,
        statement: ConstructId,
        context_id: ConstructId,
        limit: u32,
    ) -> Result<Option<Invariant>, UnresolvedConstructError> {
        let generated_invariants = self.generated_invariants(context_id)?;
        for inv in generated_invariants {
            if self.is_def_equal(
                SubExpr(statement, &Default::default()),
                SubExpr(inv.statement, &Default::default()),
                limit,
            )? == TripleBool::True
            {
                return Ok(Some(inv));
            }
        }
        let scope = self.get_construct(context_id).scope.dyn_clone();
        let inv = scope.lookup_invariant(self, statement)?;
        Ok(inv)
    }
}
