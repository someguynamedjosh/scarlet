use std::{collections::HashMap, mem::replace};

use super::{
    replace::{self, Reps},
    structure::{Definition, Environment, Item, ItemId, Variable},
};

pub fn dedup<'x>(env: &Environment<'x>) -> Environment<'x> {
    let mut new_env = Environment::new();
    let mut inserted_items = HashMap::new();
    let mut defs_to_set = Vec::new();
    let mut reps = HashMap::new();

    for (id, item) in &env.items {
        if let Some(existing) = inserted_items.get(item.definition.as_ref().unwrap()) {
            reps.insert(id, *existing);
        } else {
            let definition = None;
            let original_definition = item.original_definition;
            let new_item = Item {
                definition,
                original_definition,
            };
            let inserted = new_env.items.push(new_item);
            let original_def = item.definition.as_ref().unwrap();
            inserted_items.insert(original_def, inserted);
            defs_to_set.push((inserted, original_def));
            reps.insert(id, inserted);
        }
    }

    for (target_id, definition) in defs_to_set {
        let mut new_def = definition.clone();
        replace::apply_reps_to_def(&reps, &mut new_def);
        new_env.items[target_id].definition = Some(new_def);
    }

    for (_, var) in &env.vars {
        let mut pattern = var.pattern;
        replace::apply_reps(&reps, &mut pattern);
        new_env.vars.push(Variable { pattern });
    }

    new_env
}
