use super::{ConstructId, Environment};
use crate::{constructs::ConstructDefinition, environment::UnresolvedConstructError};

#[derive(Debug, PartialEq, Eq)]
pub struct ResolveStackFrame(ConstructId);
pub type ResolveStack = Vec<ResolveStackFrame>;

impl<'x> Environment<'x> {
    pub fn resolve_all(&mut self) {
        self.for_each_construct_returning_nothing(Self::resolve);
        self.for_each_construct_returning_nothing(|env, id| {
            let def = &env.constructs[id].definition;
            assert!(def.as_other().is_some() || def.as_resolved().is_some());
        })
    }

    pub fn resolve(&mut self, con_id: ConstructId) {
        let con = &self.constructs[con_id];
        if self.resolve_stack.contains(&ResolveStackFrame(con_id)) {
            eprintln!("{:#?}", self);
            eprintln!("{:?}", self.resolve_stack);
            todo!("Nice error, circular dependency");
        }
        if let ConstructDefinition::Unresolved(resolvable) = &con.definition {
            self.resolve_stack.push(ResolveStackFrame(con_id));
            let resolvable = resolvable.dyn_clone();
            let scope = con.scope.dyn_clone();
            let new_def = resolvable.resolve(self, scope);
            match new_def {
                Ok(new_def) => {
                    self.constructs[con_id].definition = new_def;
                    assert_eq!(self.resolve_stack.pop(), Some(ResolveStackFrame(con_id)));
                }
                Err(UnresolvedConstructError(unresolved)) => {
                    self.resolve(unresolved);
                    assert_eq!(self.resolve_stack.pop(), Some(ResolveStackFrame(con_id)));
                    self.resolve(con_id);
                }
            }
        }
    }
}
