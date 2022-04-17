use super::{Environment, ItemPtr};
use crate::item::{
    resolvable::{ResolveError, ResolveResult}, definition::ItemDefinition,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ResolveStackFrame(ItemPtr);
pub type ResolveStack = Vec<ResolveStackFrame>;

impl Environment {
    pub fn resolve_all(&mut self) {
        let mut unresolved = Vec::new();
        self.for_each_item_returning_nothing(|env, id| {
            if env.items[id].is_unresolved() {
                unresolved.push(id);
            }
        });
        let mut limit = 0;
        while limit < 16 {
            let mut reset_limit = false;
            let mut still_unresolved = Vec::new();
            let mut all_dead_ends = true;
            for id in unresolved {
                println!("Resolving {:?} limit {}", id, limit);
                let res = self.resolve(id, limit);
                if let Ok(true) = res {
                    // Right now this line actually significantly slows things
                    // down. In theory it should accelerate things. Maybe we
                    // need more complicated code for the effect to be
                    // noticable.

                    // I'm leaving this on because it fixes a bug I can't be
                    // fucked fixing properly right now.
                    reset_limit = limit != 0;
                } else {
                    if let Err(ResolveError::InvariantDeadEnd(..)) = &res {
                    } else {
                        all_dead_ends = false;
                    }
                    still_unresolved.push(id);
                }
            }
            unresolved = still_unresolved;
            if reset_limit {
                limit = 0;
            } else {
                limit += 1;
            }
            if all_dead_ends {
                break;
            }
        }
        let mut problem = false;
        self.for_each_item_returning_nothing(|env, con| {
            if let Err(err) = env.resolve(con, limit) {
                println!("Failed to resolve {:?} because", con);
                problem = true;
                match err {
                    ResolveError::Unresolved(err) => {
                        // eprintln!("{}", &format!("{:#?}", env)[0..30_000]);
                        eprintln!("{:?} relies on {:?}", con, err.0);
                        eprintln!("{:?} is {:#?}", con, env.get_item(err.0))
                    }
                    ResolveError::InvariantDeadEnd(err) => eprintln!("{}", err),
                    ResolveError::MaybeInvariantDoesNotExist => {
                        eprintln!("Recursion limit exceeded while searching for invariants")
                    }
                    ResolveError::Placeholder => eprintln!("placeholder"),
                }
            }
        });
        self.for_each_item_returning_nothing(|env, item| {
            env.generated_invariants(item);
        });
        if problem {
            panic!("Failed to resolve construct(s)");
        }
    }

    /// Returns Ok(true) if the resolution was successful, or Ok(false) if it
    /// was already resolved.
    pub fn resolve(&mut self, item_id: ItemPtr, limit: u32) -> Result<bool, ResolveError> {
        let item = &self.items[item_id];
        if self.resolve_stack.contains(&ResolveStackFrame(item_id)) {
            eprintln!("{:#?}", self);
            eprintln!("{:?}", self.resolve_stack);
            todo!("Nice error, circular dependency");
        }
        if let Some(resolvable) = &item.unresolved {
            self.resolve_stack.push(ResolveStackFrame(item_id));
            let resolvable = resolvable.dyn_clone();
            let scope = item.scope.dyn_clone();
            let new_def = resolvable.resolve(self, item_id, scope, limit);
            match new_def {
                ResolveResult::Ok(new_def) => {
                    if let ItemDefinition::Resolved(boxed) = new_def {
                        self.define_dyn_item(item_id, boxed);
                    } else {
                        self.items[item_id].definition = new_def;
                    }
                    self.arrest_recursion(item_id);
                    assert_eq!(self.resolve_stack.pop(), Some(ResolveStackFrame(item_id)));
                    self.items[item_id].unresolved = None;
                    Ok(true)
                }
                ResolveResult::Partial(new_def) => {
                    if let ItemDefinition::Resolved(boxed) = new_def {
                        self.define_dyn_item(item_id, boxed);
                    } else {
                        self.items[item_id].definition = new_def;
                    }
                    self.arrest_recursion(item_id);
                    assert_eq!(self.resolve_stack.pop(), Some(ResolveStackFrame(item_id)));
                    Err(ResolveError::Placeholder)
                }
                ResolveResult::Err(err) => {
                    assert_eq!(self.resolve_stack.pop(), Some(ResolveStackFrame(item_id)));
                    Err(err)
                }
            }
        } else if let ItemDefinition::Placeholder = &item.definition {
            return Err(ResolveError::Placeholder);
        } else {
            Ok(false)
        }
    }
}
