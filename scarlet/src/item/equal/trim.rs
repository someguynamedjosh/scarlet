use super::equal::Equal;
use crate::{item::ItemId, util::PtrExtension};

// x -> x
// fx -> gy(y IS x)(x IS x)

pub(super) fn trim_result(result: &mut Equal) {
    match result {
        Equal::Yes(left, right) => trim_yes(left, right),
        _ => (),
    }
}

fn trim_yes(left: &mut Substitutions, right: &mut Substitutions) {
    mostly_trim_yes(left, right);
    thoroughly_remove_identity_substitutions(left);
    thoroughly_remove_identity_substitutions(right);
}

/// Uses equality checking to find identity substitutions instead of just
/// checking for identical pointers.
fn thoroughly_remove_identity_substitutions(subs: &mut Substitutions) {
    for (target, value) in subs.clone() {
        let mut result = value.get_equality_left(target.borrow().item())?;
        // Skip the last step to prevent infinite cycles.
        mostly_trim_result(&mut result);
        if result.is_trivial_yes() {
            subs.remove(&target);
        }
    }
    Ok(())
}

// Skips the final step.
fn mostly_trim_result(result: &mut Equal) {
    match result {
        Equal::Yes(left, right) => mostly_trim_yes(left, right),
        _ => (),
    }
}

// Skips the final step.
fn mostly_trim_yes(left: &mut Substitutions, right: &mut Substitutions) {
    trim_substitutions(left);
    trim_substitutions(right);
    for (target, left_value) in left.clone() {
        if let Some(right_value) = right.get(&target) {
            if left_value
                .dereference()
                .is_same_instance_as(&right_value.dereference())
            {
                left.remove(&target);
                right.remove(&target);
                continue;
            }
        }
    }
    for (target, right_value) in right.clone() {
        if let Some(left_value) = left.get(&target) {
            if left_value
                .dereference()
                .is_same_instance_as(&right_value.dereference())
            {
                left.remove(&target);
                right.remove(&target);
                continue;
            }
        }
    }
}

fn trim_substitutions(substitutions: &mut Substitutions) {
    for (_, item) in substitutions.iter_mut() {
        *item = trim_item(item);
    }
    remove_identity_substitutions(substitutions);
}

fn remove_identity_substitutions(substitutions: &mut Substitutions) {
    let mut filtered = Substitutions::new();
    for (target, value) in substitutions.iter_mut() {
        let target_d = target.borrow().item().dereference();
        let value_d = value.dereference();
        if !target_d.is_same_instance_as(&value_d) {
            filtered.insert_no_replace(target.ptr_clone(), value_d);
        }
    }
    *substitutions = filtered;
}

#[must_use]
fn trim_item(item: &ItemId) -> ItemId {
    let item = item.dereference();
    if let Some(mut sub_item) = item.downcast_definition_mut::<DSubstitution>() {
        let mut new_base = sub_item.base().ptr_clone();
        if let Some(subs) = sub_item.substitutions_mut() {
            new_base = trim_item(&new_base).dereference();
            trim_substitutions(subs);
            for (target, value) in &*subs {
                if target
                    .borrow()
                    .item()
                    .dereference()
                    .is_same_instance_as(&new_base)
                    && target.borrow().dependencies().len() == 0
                {
                    return value.ptr_clone();
                }
            }
            if subs.len() == 0 {
                return new_base;
            }
        }
        if let Some(base) = sub_item.base_mut() {
            *base = new_base;
        }
    }
    item
}
