use var_list::VarList;

use crate::{
    shared::ItemId,
    stage3::structure::{self as stage3},
    stage4::structure::Environment,
};

mod compute_type;
mod dependencies;
pub mod var_list;

fn compute_types(env: &mut Environment) -> Result<(), String> {
    let mut next_item = ItemId(0);
    while next_item.0 < env.items.len() {
        env.compute_type(next_item, vec![]).into_option_or_err()?;
        next_item.0 += 1;
    }
    Ok(())
}

fn count_computed_types(env: &Environment) -> usize {
    env.items.iter().filter(|i| i.typee.is_some()).count()
}

pub fn ingest(from: stage3::Environment) -> Result<Environment, String> {
    let mut env = Environment::new(from);
    let mut previous_computed_types = count_computed_types(&env);
    loop {
        compute_types(&mut env)?;
        let computed_types = count_computed_types(&env);
        let target = env.items.len();
        if computed_types == target {
            break;
        } else if computed_types > previous_computed_types {
            previous_computed_types = computed_types;
            continue;
        } else {
            println!("{:#?}", env);
            panic!("Type elaboration has stalled");
        }
    }
    Ok(env)
}
