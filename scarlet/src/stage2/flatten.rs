use std::collections::HashMap;

use super::{
    replace::{self, Reps},
    structure::{Definition, Environment, ItemId},
};

fn flatten_id<'x>(reps: &Reps<'x>, id: ItemId<'x>) -> ItemId<'x> {
    if let Some(&replaced_with) = reps.get(&id) {
        if replaced_with == id {
            replaced_with
        } else {
            flatten_id(reps, replaced_with)
        }
    } else {
        id
    }
}

pub fn flatten(env: &mut Environment) {
    let mut reps = HashMap::new();

    for (id, item) in &env.items {
        if let Definition::Other(replace_with) = item.definition.as_ref().unwrap() {
            reps.insert(id, *replace_with);
        }
    }

    let mut flat_reps = HashMap::new();
    for (&target, &replace_with) in &reps {
        let replace_with = flatten_id(&reps, replace_with);
        flat_reps.insert(target, replace_with);
    }

    for (_, item) in &mut env.items {
        replace::apply_reps_to_def(&reps, item.definition.as_mut().unwrap());
    }
    for (_, var) in &mut env.vars {
        replace::apply_reps(&reps, &mut var.pattern);
    }
}
