mod diagnostics;
mod matchh;
mod node;
mod parse;
mod phrase;
mod scarlet_phrases;
mod stack;
mod util;

pub use node::{Node, NodeChild};
pub use parse::{parse_tree, ParseContext};

use self::phrase::CreateContext;
use crate::{diagnostic::Diagnostic, environment::Environment, item::ItemPtr, scope::SRoot};

pub fn create_root(
    node: &Node,
    pc: &ParseContext,
    env: &mut Environment,
) -> Result<ItemPtr, Diagnostic> {
    let mut ctx = CreateContext { pc, env };
    node.as_item_dyn_scope(&mut ctx, Box::new(SRoot))
}
