use std::fmt::Debug;

use super::{
    node::Node,
    phrase::{PhraseTable, Precedence},
};
use crate::{
    diagnostic::{Diagnostic, Position},
    parser::node::NodeChild,
};

pub fn incomplete_phrase_error(node: &Node) -> Diagnostic {
    Diagnostic::new()
        .with_text_error(format!(
            "This looks like a \"{}\" phrase, but it is incomplete:",
            node.phrase
        ))
        .with_source_code_block_error(node.position)
}

pub fn unrecognized_input(position: Position) -> Diagnostic {
    Diagnostic::new()
        .with_text_error("Not sure what to do with the following input:".to_owned())
        .with_source_code_block_error(position)
}
