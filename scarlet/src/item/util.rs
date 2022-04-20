use super::{
    definitions::{
        decision::DDecision,
        substitution::{DSubstitution, Substitutions},
    },
    Item, ItemPtr,
};
use crate::{environment::Environment, scope::SRoot};

pub fn unchecked_substitution(base: ItemPtr, subs: &Substitutions) -> ItemPtr {
    if subs.len() == 0 {
        return base;
    } else if subs.len() == 1  {
        let (target, value) = subs.iter().next().unwrap();
        if target.borrow().item().is_same_instance_as(&base) {
            return value.ptr_clone()
        }
    }
    let scope = base.clone_scope();
    let def = DSubstitution::new_unchecked(base, subs.clone());
    Item::new_boxed(Box::new(def), scope)
}

pub fn decision(
    left: ItemPtr,
    right: ItemPtr,
    when_equal: ItemPtr,
    when_not_equal: ItemPtr,
) -> ItemPtr {
    let scope = left.clone_scope();
    let def = DDecision::new(left, right, when_equal, when_not_equal);
    Item::new_boxed(Box::new(def), scope)
}

pub fn equals(env: &Environment, left: ItemPtr, right: ItemPtr) -> ItemPtr {
    let truee = env.get_true().ptr_clone();
    let falsee = env.get_false().ptr_clone();
    decision(left, right, truee, falsee)
}

pub fn is_bool(env: &Environment, item_to_test: ItemPtr) -> ItemPtr {
    let truee = env.get_true().ptr_clone();
    let falsee = env.get_false().ptr_clone();
    let item_is_false = equals(env, item_to_test.ptr_clone(), falsee);
    decision(
        item_to_test.ptr_clone(),
        truee.ptr_clone(),
        truee.ptr_clone(),
        item_is_false,
    )
}

pub fn placeholder() -> ItemPtr {
    let def: DSubstitution = todo!("DPlaceholder");
    Item::new(def, SRoot)
}
