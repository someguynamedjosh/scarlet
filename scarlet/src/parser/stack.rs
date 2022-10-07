use std::fmt::Debug;

use super::{
    node::Node,
    phrase::{PhraseTable, Precedence},
};
use crate::{
    diagnostic::Diagnostic,
    parser::{diagnostics::incomplete_phrase_error, node::NodeChild},
};

#[derive(Debug)]
pub struct Stack<'a>(pub Vec<Node<'a>>);

impl<'a> Stack<'a> {
    pub fn collapse(&mut self, pt: &PhraseTable) -> Result<(), Diagnostic> {
        assert!(
            self.0.len() >= 2,
            "Internal error, tried to collapse with <2 items on the stack."
        );
        let top = self.0.pop().unwrap();
        if top.is_complete(pt) {
            let next = self.0.len() - 1;
            let next = &mut self.0[next];
            assert!(next.is_waiting_for_node(pt));
            next.children.push(NodeChild::Node(top));
            Ok(())
        } else {
            Err(incomplete_phrase_error(&top))
        }
    }

    /// Collapses the stack until collapsing it more would result in the top
    /// item having a higher precedence than `prec`.
    pub fn collapse_to_precedence(
        &mut self,
        pt: &PhraseTable,
        prec: Precedence,
    ) -> Result<(), Diagnostic> {
        while self.0.len() >= 2
            && self.0[self.0.len() - 1].is_complete(pt)
            && pt[self.0[self.0.len() - 2].phrase].precedence <= prec
            && !self.0[self.0.len() - 2].will_wait_for_text(pt)
        {
            self.collapse(pt)?;
        }
        Ok(())
    }
}
