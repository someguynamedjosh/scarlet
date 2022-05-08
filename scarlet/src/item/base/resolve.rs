use crate::{
    environment::Environment,
    item::{
        definitions::placeholder::DPlaceholder,
        resolvable::{DResolvable, ResolveError, ResolveResult},
        ItemPtr,
    },
};

pub fn resolve_all(env: &mut Environment, root: ItemPtr) {
    let mut unresolved = Vec::new();
    root.for_self_and_deep_contents(&mut |item| {
        if item.is_unresolved() {
            unresolved.push(item.ptr_clone());
        }
    });
    let mut limit = 0;
    while limit < 16 {
        let mut reset_limit = false;
        let mut still_unresolved = Vec::new();
        let mut all_dead_ends = true;
        for id in unresolved {
            println!("Resolving {:?} limit {}", id.debug_label(), limit);
            let res = resolve(env, id.ptr_clone(), limit);
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
    root.for_self_and_deep_contents(&mut |item| {
        if let Err(err) = resolve(env, item.ptr_clone(), limit) {
            println!("Failed to resolve {:#?} because", item);
            problem = true;
            match err {
                ResolveError::Unresolved(err) => {
                    // eprintln!("{}", &format!("{:#?}", env)[0..30_000]);
                    eprintln!("it relies on {:#?}", err.0);
                }
                ResolveError::InvariantDeadEnd(err) => eprintln!("{}", err),
                ResolveError::MaybeInvariantDoesNotExist => {
                    eprintln!("Recursion limit exceeded while searching for invariants")
                }
                ResolveError::Placeholder => eprintln!("placeholder"),
            }
        }
    });
    if problem {
        panic!("Failed to resolve construct(s)");
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
                item.mark_recursion();
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
