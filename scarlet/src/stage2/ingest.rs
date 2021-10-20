mod from_tree;
mod top_level;
mod util;

use super::{flatten, structure::Environment};
use crate::stage1::structure::Module;

pub fn ingest<'x>(src: &'x Module) -> Environment<'x> {
    let mut env = top_level::ingest(src);
    flatten::flatten(&mut env);
    env
}
