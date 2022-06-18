use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    file_tree::FileNode,
    item::{
        definitions::placeholder::DPlaceholder,
        resolvable::{DResolvable, ResolveError, ResolveResult},
        ItemPtr,
    },
};

pub fn resolve_all(env: &mut Environment, root: ItemPtr) -> Result<(), Vec<Diagnostic>> {
    let mut unresolved = HashSet::new();
    root.for_self_and_deep_contents(&mut |item| {
        if item.is_unresolved() {
            unresolved.insert(item.ptr_clone());
        }
    });
    let mut limit = 0;
    while limit < 16 {
        let mut reset_limit = false;
        let mut still_unresolved = HashSet::new();
        let mut all_dead_ends = true;
        for id in unresolved {
            assert!(id.is_unresolved());
            let res = resolve(env, id.ptr_clone(), limit);
            if let Ok(true) = res {
                // Right now this line actually significantly slows things
                // down. In theory it should accelerate things. Maybe we
                // need more complicated code for the effect to be
                // noticable.
                // reset_limit = limit != 0;
                still_unresolved.remove(&id);
                id.for_self_and_deep_contents(&mut |item| {
                    if item.is_unresolved() {
                        still_unresolved.insert(item.ptr_clone());
                    }
                });
            } else {
                if let Err(ResolveError::InvariantDeadEnd(..)) = &res {
                } else {
                    all_dead_ends = false;
                }
                assert!(id.is_unresolved());
                still_unresolved.insert(id);
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
    let mut problems = Vec::new();
    root.for_self_and_deep_contents(&mut |item| {
        if let Err(err) = resolve(env, item.ptr_clone(), limit) {
            let diagnostic = match err {
                ResolveError::Unresolved(err) => {
                    todo!("Nice error, it relies on {:#?}", err.0);
                }
                ResolveError::InvariantDeadEnd(err) => todo!("Nice error, {}", err),
                ResolveError::MaybeInvariantDoesNotExist => {
                    todo!("Nice error, Recursion limit exceeded while searching for invariants")
                }
                ResolveError::Placeholder => todo!("Nice error, placeholder"),
                ResolveError::Diagnostic(diagnostic) => diagnostic,
            };
            problems.push(diagnostic);
        }
    });
    if problems.len() == 0 {
        Ok(())
    } else {
        Err(problems)
    }
}

/// Returns Ok(true) if the resolution was successful, or Ok(false) if it
/// was already resolved.
fn resolve(env: &mut Environment, item: ItemPtr, limit: u32) -> Result<bool, ResolveError> {
    if let Some(wrapper) = item.downcast_definition::<DResolvable>() {
        let scope = item.clone_scope();
        let new_def = wrapper
            .resolvable()
            .resolve(env, item.ptr_clone(), scope, limit);
        drop(wrapper);
        match new_def {
            ResolveResult::Ok(new_def) => {
                item.redefine(new_def);
                Ok(true)
            }
            ResolveResult::Err(err) => Err(err),
        }
    } else if let Some(..) = item.downcast_definition::<DPlaceholder>() {
        return Err(ResolveError::Placeholder);
    } else {
        Ok(false)
    }
}
