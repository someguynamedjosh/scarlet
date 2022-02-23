use super::{ConstructId, Environment, UnresolvedConstructError};
use crate::{
    constructs::{
        base::BoxedConstruct, downcast_construct, AnnotatedConstruct, Construct,
        ConstructDefinition,
    },
    scope::{LookupIdentResult, Scope},
};

impl<'x> Environment<'x> {
    pub fn get_construct(&self, con_id: ConstructId) -> &AnnotatedConstruct<'x> {
        &self.constructs[con_id]
    }

    pub fn get_construct_scope(&mut self, con_id: ConstructId) -> &dyn Scope {
        let con = self.get_construct(con_id);
        match &con.definition {
            &ConstructDefinition::Other(other) => self.get_construct_scope(other),
            ConstructDefinition::Resolved(_) => &*self.get_construct(con_id).scope,
            ConstructDefinition::Unresolved(_) => unreachable!(),
        }
    }

    pub fn lookup_ident(&mut self, con_id: ConstructId, ident: &str) -> LookupIdentResult {
        self.constructs[con_id]
            .scope
            .dyn_clone()
            .lookup_ident(self, ident)
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
        Ok(downcast_construct(
            &**self.get_construct_definition_no_deref(con_id)?,
        ))
    }

    pub fn get_construct_definition(
        &mut self,
        con_id: ConstructId,
    ) -> Result<&BoxedConstruct, UnresolvedConstructError> {
        let _old_con_id = con_id;
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
}
