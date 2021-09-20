use crate::{
    stage2::structure::{Item, ItemId},
    stage3::Environment,
};
use std::collections::HashMap;

mod basics;
mod reduce;

pub fn simplify(env: Environment) -> Environment {
    let mut env = env;
    basics::remove_duplicates(&mut env);
    basics::flatten_members(&mut env);
    basics::remove_unused(&mut env);
    env
}

fn apply_replacements_to_id(target: &mut ItemId, replacements: &HashMap<ItemId, ItemId>) {
    if let Some(rep) = replacements.get(target) {
        *target = *rep;
    }
}

fn apply_replacements_to_item(target: &mut Item, replacements: &HashMap<ItemId, ItemId>) {
    match target {
        Item::Defining { base, definitions } => {
            apply_replacements_to_id(base, replacements);
            for (_, def) in definitions {
                apply_replacements_to_id(def, replacements);
            }
        }
        Item::GodType => (),
        Item::FromType { base, vars } => {
            apply_replacements_to_id(base, replacements);
            for var in vars {
                apply_replacements_to_id(var, replacements);
            }
        }
        Item::InductiveType(id) => apply_replacements_to_id(id, replacements),
        Item::InductiveValue { typee, records, .. } => {
            apply_replacements_to_id(typee, replacements);
            for record in records {
                apply_replacements_to_id(record, replacements);
            }
        }
        Item::Item(id) => apply_replacements_to_id(id, replacements),
        Item::Member { base, .. } => apply_replacements_to_id(base, replacements),
        Item::PrimitiveType(..) | Item::PrimitiveValue(..) => (),
        Item::Public(id) => apply_replacements_to_id(id, replacements),
        Item::Replacing {
            base,
            replacements: user_specified_replacements,
        } => {
            apply_replacements_to_id(base, replacements);
            for (target, value) in user_specified_replacements {
                apply_replacements_to_id(target, replacements);
                apply_replacements_to_id(value, replacements);
            }
        }
        Item::Variable { selff, typee, .. } => {
            apply_replacements_to_id(selff, replacements);
            apply_replacements_to_id(typee, replacements);
        }
    }
}

fn apply_replacements(env: &mut Environment, replacements: &HashMap<ItemId, ItemId>) {
    for (_, item, typee) in env.iter_mut() {
        if let Some(tid) = typee {
            apply_replacements_to_id(tid, replacements);
        }
        apply_replacements_to_item(item, replacements)
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
