mod tests;

use std::collections::HashSet;

use super::{
    dependencies::DepResStackFrame, discover_equality::Equal, ConstructId, Environment,
    UnresolvedConstructError,
};
use crate::{
    constructs::{
        base::BoxedConstruct, downcast_construct, AnnotatedConstruct, Construct,
        ConstructDefinition, GenInvResult,
    },
    environment::sub_expr::SubExpr,
    scope::{LookupInvariantError, LookupInvariantResult, LookupSimilarInvariantResult, Scope},
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invariant {
    pub statement: ConstructId,
    pub dependencies: HashSet<ConstructId>,
}

impl Invariant {
    pub fn new(statement: ConstructId, dependencies: HashSet<ConstructId>) -> Self {
        Self {
            statement,
            dependencies,
        }
    }
}

impl<'x> Environment<'x> {
    pub fn generated_invariants(&mut self, con_id: ConstructId) -> GenInvResult {
        for frame in &self.dep_res_stack {
            if frame.0 == con_id {
                return Vec::new();
            }
        }

        self.dep_res_stack.push(DepResStackFrame(con_id));
        let context = match self.get_construct_definition(con_id) {
            Ok(ok) => ok,
            Err(_err) => {
                self.dep_res_stack.pop();
                return Vec::new();
            }
        };
        let context = context.dyn_clone();
        let invs = context.generated_invariants(con_id, self);
        self.constructs[con_id].invariants = Some(invs.clone());
        self.dep_res_stack.pop();
        invs
    }

    pub fn get_produced_invariant(
        &mut self,
        statement: ConstructId,
        context_id: ConstructId,
        limit: u32,
    ) -> LookupSimilarInvariantResult {
        let generated_invariants = self.generated_invariants(context_id);
        for inv in generated_invariants {
            if let Ok(Equal::Yes(l, r)) = self.discover_equal(inv.statement, statement, limit) {
                if r.len() == 0 {
                    return Ok((inv, Equal::Yes(l, r)));
                } else {
                    continue;
                }
            }
        }
        let scope = self.get_construct(context_id).scope.dyn_clone();
        scope.lookup_invariant_limited(self, statement, limit)
    }

    pub fn justify(
        &mut self,
        statement: ConstructId,
        context: ConstructId,
        limit: u32,
    ) -> LookupInvariantResult {
        todo!()
    }
}
