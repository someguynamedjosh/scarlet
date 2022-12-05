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
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{ItemPtr, ResolvableItemEnum},
};

pub fn create_root(
    node: &Node,
    pc: &ParseContext,
    env: &mut Environment,
) -> Result<ItemPtr<ResolvableItemEnum>, Diagnostic> {
    let mut ctx = CreateContext { pc, env };
    node.as_item(&mut ctx)
}
