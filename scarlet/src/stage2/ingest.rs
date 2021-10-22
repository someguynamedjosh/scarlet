mod from_tree;
mod top_level;
mod util;

use super::{
    dedup, flatten,
    structure::{Environment, ItemId},
};
use crate::stage1::structure::Module;

pub fn ingest<'x>(src: &'x Module) -> (Environment<'x>, ItemId<'x>) {
    let (mut env, mut root) = top_level::ingest(src);
    flatten::flatten(&mut env);
    loop {
        let old_len = env.items.len();
        let (new_env, new_root) = dedup::dedup(env, root);
        env = new_env;
        root = new_root;
        if env.items.len() == old_len {
            break;
        }
    }
    let root = env.reduce(root);
    env.get_deps(root);
    (env, root)
}
