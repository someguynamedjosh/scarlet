use std::collections::HashMap;

use crate::{
    stage2::{Item, ItemId, Value},
    stage3::Environment,
};

pub fn simplify(env: Environment) -> Environment {
    let mut env = env;
    remove_duplicates(&mut env);
    flatten_members(&mut env);
    remove_unused(&mut env);
    env
}

fn apply_replacements_to(target: &mut ItemId, replacements: &HashMap<ItemId, ItemId>) {
    if let Some(rep) = replacements.get(target) {
        *target = *rep;
    }
}

fn apply_replacements(env: &mut Environment, replacements: &HashMap<ItemId, ItemId>) {
    for (_, item, typee) in env.iter_mut() {
        if let Some(tid) = typee {
            apply_replacements_to(tid, replacements);
        }
        match item {
            Item::Defining { base, definitions } => {
                apply_replacements_to(base, replacements);
                for (_, def) in definitions {
                    apply_replacements_to(def, replacements);
                }
            }
            Item::FromType { base, vars } => {
                apply_replacements_to(base, replacements);
                for var in vars {
                    apply_replacements_to(var, replacements);
                }
            }
            Item::InductiveValue { typee, records, .. } => {
                apply_replacements_to(typee, replacements);
                for record in records {
                    apply_replacements_to(record, replacements);
                }
            }
            Item::Item(id) => apply_replacements_to(id, replacements),
            Item::Member { base, .. } => apply_replacements_to(base, replacements),
            Item::Public(id) => apply_replacements_to(id, replacements),
            Item::Replacing {
                base,
                replacements: user_specified_replacements,
            } => {
                apply_replacements_to(base, replacements);
                for (target, value) in user_specified_replacements {
                    apply_replacements_to(target, replacements);
                    apply_replacements_to(value, replacements);
                }
            }
            Item::Value(val) => match val {
                Value::InductiveType(id) => apply_replacements_to(id, replacements),
                _ => (),
            },
            Item::Variable { selff, typee, .. } => {
                apply_replacements_to(selff, replacements);
                apply_replacements_to(typee, replacements);
            }
        }
    }
}

fn get_replacement(original: ItemId, replacements: &HashMap<ItemId, ItemId>) -> ItemId {
    if let Some(replacement) = replacements.get(&original) {
        get_replacement(*replacement, replacements)
    } else {
        original
    }
}

/// E.G. If you give id{3} -> id{4}, id{4} -> id{5}, then it will become id{3} -> id{5}, id{4} -> id{5}.
fn flatten_replacements(replacements: HashMap<ItemId, ItemId>) -> HashMap<ItemId, ItemId> {
    let mut flattened = HashMap::new();
    for key in replacements.keys() {
        flattened.insert(*key, get_replacement(*key, &replacements));
    }
    flattened
}

fn remove_duplicates(env: &mut Environment) {
    let mut map = HashMap::new();
    let mut replacements = HashMap::new();
    for (id, item, _typee) in env.iter_mut() {
        if let Some(replacement) = map.get(item) {
            replacements.insert(id, *replacement);
        } else {
            map.insert(item.clone(), id);
        }
    }
    apply_replacements(env, &replacements);
}

fn flatten_members(env: &mut Environment) {
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
    let replacements = flatten_replacements(replacements);
    apply_replacements(env, &replacements);
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
        Item::Item(id) => used[id.0] = true,
        Item::InductiveValue { typee, records, .. } => {
            used[typee.0] = true;
            for record in records {
                used[record.0] = true;
            }
        }
        Item::Member { base, .. } => used[base.0] = true,
        Item::Public(id) => used[id.0] = true,
        Item::Replacing { base, replacements } => {
            used[base.0] = true;
            for (target, value) in replacements {
                used[target.0] = true;
                used[value.0] = true;
            }
        }
        Item::Value(val) => match val {
            Value::InductiveType(id) => used[id.0] = true,
            _ => (),
        },
        Item::Variable { selff, typee } => {
            used[selff.0] = true;
            used[typee.0] = true;
        }
    }
}

fn remove_unused(env: &mut Environment) {
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
    apply_replacements(env, &replacements);
}
