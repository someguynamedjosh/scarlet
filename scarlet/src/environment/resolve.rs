use super::{ConstructId, Environment};
use crate::{constructs::ConstructDefinition, resolvable::ResolveError, util::Ignorable};

#[derive(Debug, PartialEq, Eq)]
pub struct ResolveStackFrame(ConstructId);
pub type ResolveStack = Vec<ResolveStackFrame>;

impl<'x> Environment<'x> {
    pub fn resolve_all(&mut self) {
        for limit in 0..128 {
            self.for_each_construct_returning_nothing(|env, con| env.resolve(con, limit).ignore());
        }
        let mut problem = false;
        self.for_each_construct_returning_nothing(|env, con| {
            if let Err(err) = env.resolve(con, 128) {
                problem = true;
                match err {
                    ResolveError::UnresolvedConstruct(err) => {
                        eprintln!("{:?} relies on {:?}", con, err.0)
                    }
                    ResolveError::InsufficientInvariants(err) => eprintln!("{}", err),
                }
            }
        });
        if problem {
            panic!("Failed to resolve construct(s)");
        }
    }

    pub fn resolve(&mut self, con_id: ConstructId, limit: u32) -> Result<(), ResolveError> {
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
            let new_def = resolvable.resolve(self, scope, limit);
            match new_def {
                Ok(new_def) => {
                    self.constructs[con_id].definition = new_def;
                    assert_eq!(self.resolve_stack.pop(), Some(ResolveStackFrame(con_id)));
                    Ok(())
                }
                Err(err) => {
                    assert_eq!(self.resolve_stack.pop(), Some(ResolveStackFrame(con_id)));
                    Err(err)
                }
            }
        } else {
            Ok(())
        }
    }
}
