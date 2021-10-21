mod from_tree;
mod top_level;
mod util;

use super::{dedup, flatten, structure::Environment};
use crate::stage1::structure::Module;

pub fn ingest<'x>(src: &'x Module) -> Environment<'x> {
    let mut env = top_level::ingest(src);
    flatten::flatten(&mut env);
    loop {
        let old_len = env.items.len();
        let new_env = dedup::dedup(env);
        env = new_env;
        if env.items.len() == old_len {
            break;
        }
    }
    env.find_all_dependencies();
    env
}
