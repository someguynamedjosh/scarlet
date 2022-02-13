use super::{ConstructId, Environment};
use crate::{constructs::ConstructDefinition, resolvable::ResolveError, util::Ignorable};

#[derive(Debug, PartialEq, Eq)]
pub struct ResolveStackFrame(ConstructId);
pub type ResolveStack = Vec<ResolveStackFrame>;

impl<'x> Environment<'x> {
    pub fn resolve_all(&mut self) {
        let mut unresolved = Vec::new();
        self.for_each_construct_returning_nothing(|env, id| {
            if env.constructs[id].definition.is_unresolved() {
                unresolved.push(id);
            }
        });
        let mut limit = 0;
        while limit < 128 {
            let reset_limit = false;
            let mut still_unresolved = Vec::new();
            for id in unresolved {
                if reset_limit {
                    still_unresolved.push(id);
                    continue;
                }
                let res = self.resolve(id, limit);
                if let Ok(true) = res {
                    // Right now this line actually significantly slows things
                    // down. In theory it should accelerate things. Maybe we
                    // need more complicated code for the effect to be
                    // noticable. reset_limit = limit != 0;
                } else {
                    still_unresolved.push(id);
                }
            }
            unresolved = still_unresolved;
            if reset_limit {
                limit = 0;
            } else {
                limit += 1;
            }
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

    /// Returns Ok(true) if the resolution was successful, or Ok(false) if it
    /// was already resolved.
    pub fn resolve(&mut self, con_id: ConstructId, limit: u32) -> Result<bool, ResolveError> {
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
                    Ok(true)
                }
                Err(err) => {
                    assert_eq!(self.resolve_stack.pop(), Some(ResolveStackFrame(con_id)));
                    Err(err)
                }
            }
        } else {
            Ok(false)
        }
    }
}
