use super::{
    definitions::{
        builtin_function::DBuiltinFunction,
        placeholder::DPlaceholder,
        substitution::{DSubstitution, Substitutions},
    },
    resolvable::DResolvable,
    resolve, Item, ItemPtr,
};
use crate::{diagnostic::Position, environment::Environment, scope::SRoot};

pub fn unchecked_substitution(base: ItemPtr, subs: &Substitutions) -> ItemPtr {
    if subs.len() == 0 {
        return base;
    } else if subs.len() == 1 {
        let (target, value) = subs.iter().next().unwrap();
        if target
            .borrow()
            .item()
            .is_same_instance_as(&base.dereference())
        {
            return value.ptr_clone();
        }
    }
    unchecked_substitution_without_shortcuts(base, subs)
}

/// Unlike unchecked_substitution, does not simplify cases like abc[] or def[def
/// IS ghjkl]
pub fn unchecked_substitution_without_shortcuts(base: ItemPtr, subs: &Substitutions) -> ItemPtr {
    let scope = base.clone_scope();
    let def = DSubstitution::new_unchecked(base, subs.clone());
    Item::new_boxed(Box::new(def), scope)
}

pub fn decision(
    env: &mut Environment,
    left: ItemPtr,
    right: ItemPtr,
    when_equal: ItemPtr,
    when_not_equal: ItemPtr,
) -> ItemPtr {
    let scope = left.clone_scope();
    let def = DResolvable::new(DBuiltinFunction::decision(
        env,
        left,
        right,
        when_equal,
        when_not_equal,
        Box::new(SRoot),
        Position::placeholder(),
    ));
    let item = Item::new_boxed(Box::new(def), scope);
    resolve::resolve_all(env, item.ptr_clone()).unwrap();
    item
}

pub fn equals(env: &mut Environment, left: ItemPtr, right: ItemPtr) -> ItemPtr {
    let truee = env.get_true().ptr_clone();
    let falsee = env.get_false().ptr_clone();
    decision(env, left, right, truee, falsee)
}

pub fn is_bool(env: &mut Environment, item_to_test: ItemPtr) -> ItemPtr {
    let truee = env.get_true().ptr_clone();
    let falsee = env.get_false().ptr_clone();
    let item_is_false = equals(env, item_to_test.ptr_clone(), falsee);
    decision(
        env,
        item_to_test.ptr_clone(),
        truee.ptr_clone(),
        truee.ptr_clone(),
        item_is_false,
    )
}

pub fn placeholder() -> ItemPtr {
    Item::placeholder_with_scope(format!("placeholder for test"), Box::new(SRoot))
}
