use crate::{
    stage2::structure::{Item, ItemId},
    stage3::Environment,
};
use std::collections::HashMap;

pub(super) fn remove_duplicates(env: &mut Environment) {
    let mut map = HashMap::new();
    let mut replacements = HashMap::new();
    for (id, item, _typee) in env.iter_mut() {
        if let Some(replacement) = map.get(item) {
            replacements.insert(id, *replacement);
        } else {
            map.insert(item.clone(), id);
        }
    }
    super::apply_replacements(env, &replacements);
}

pub(super) fn flatten_members(env: &mut Environment) {
    let mut to_process = Vec::new();
    for (id, item, _typee) in env.iter() {
        if let Item::Member { base, name } = item {
            to_process.push((id, *base, name.clone()))
        }
    }

    let mut replacements = HashMap::new();
    for (id, base, name) in to_process {
        replacements.insert(id, env.get_member(base, &name).unwrap());
    }
    let replacements = super::flatten_replacements(replacements);
    super::apply_replacements(env, &replacements);
}

fn mark_used_items(used_by: &Item, used: &mut [bool]) {
    match used_by {
        Item::Defining { base, definitions } => {
            used[base.0] = true;
            for (_, def) in definitions {
                used[def.0] = true;
            }
        }
        Item::FromType { base, vars } => {
            used[base.0] = true;
            for var in vars {
                used[var.0] = true;
            }
        }
        Item::GodType => (),
        Item::Item(id) => used[id.0] = true,
        Item::InductiveType(id) => used[id.0] = true,
        Item::InductiveValue { typee, records, .. } => {
            used[typee.0] = true;
            for record in records {
                used[record.0] = true;
            }
        }
        Item::Member { base, .. } => used[base.0] = true,
        Item::PrimitiveType(..) | Item::PrimitiveValue(..) => (),
        Item::Public(id) => used[id.0] = true,
        Item::Replacing { base, replacements } => {
            used[base.0] = true;
            for (target, value) in replacements {
                used[target.0] = true;
                used[value.0] = true;
            }
        }
        Item::Variable { selff, typee } => {
            used[selff.0] = true;
            used[typee.0] = true;
        }
    }
}

pub(super) fn remove_unused(env: &mut Environment) {
    let mut used = vec![false; env.iter().count()];
    for (id, item, typee) in env.iter() {
        if let Some(tid) = typee {
            used[tid.0] = true;
        }
        if env.modules.iter().any(|m| m == &id) {
            used[id.0] = true;
        }
        mark_used_items(item, &mut used[..])
    }

    let mut new_id = ItemId(0);
    let mut replacements = HashMap::new();
    let mut new_env = Environment::new_empty();
    for (id, item, typee) in env.iter() {
        if used[id.0] {
            replacements.insert(id, new_id);
            assert_eq!(new_env.insert(item.clone()), new_id);
            if let Some(typee) = typee {
                new_env.set_type(new_id, typee.clone());
            }
            new_id.0 += 1;
        }
    }

    *env = new_env;
    super::apply_replacements(env, &replacements);
}
