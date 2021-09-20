use crate::{
    stage2::{Item, ItemId, Value},
    stage3::Environment,
};
use std::collections::HashMap;

mod basics;

pub fn simplify(env: Environment) -> Environment {
    let mut env = env;
    basics::remove_duplicates(&mut env);
    basics::flatten_members(&mut env);
    basics::remove_unused(&mut env);
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
