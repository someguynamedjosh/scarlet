use std::fmt::Debug;

use super::{
    node::Node,
    phrase::{PhraseTable, Precedence},
};
use crate::parser::node::NodeChild;

#[derive(Debug)]
pub struct Stack<'a>(pub Vec<Node<'a>>);

impl<'a> Stack<'a> {
    pub fn collapse(&mut self, pt: &PhraseTable) {
        assert!(self.0.len() >= 2);
        let top = self.0.pop().unwrap();
        assert!(top.is_complete(pt));
        let next = self.0.len() - 1;
        let next = &mut self.0[next];
        assert!(next.is_waiting_for_node(pt));
        next.children.push(NodeChild::Node(top));
    }

    /// Collapses the stack until collapsing it more would result in the top
    /// item having a higher precedence than `prec`.
    pub fn collapse_to_precedence(&mut self, pt: &PhraseTable, prec: Precedence) {
        while self.0.len() >= 2
            && self.0[self.0.len() - 1].is_complete(pt)
            && pt[self.0[self.0.len() - 2].phrase].precedence <= prec
            && !self.0[self.0.len() - 2].will_wait_for_text(pt)
        {
            self.collapse(pt)
        }
    }
}
