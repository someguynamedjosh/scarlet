use std::fmt::{self, Debug, Formatter};

use typed_arena::Arena;

use super::rule::{PhraseTable, Precedence};
use crate::{constructs::ConstructId, environment::Environment, scope::Scope};

pub type CreateFn = for<'x> fn(&mut Environment<'x>, Box<dyn Scope>, &Node<'x>) -> ConstructId;

#[derive(Clone, Debug)]
pub enum NodeChild<'a> {
    Node(Node<'a>),
    Text(&'a str),
}

impl<'a> NodeChild<'a> {
    pub fn as_text(&self) -> &'a str {
        match self {
            NodeChild::Node(_) => panic!("Expected text, got a node instead"),
            NodeChild::Text(text) => text,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node<'a> {
    pub role: &'static str,
    pub children: Vec<NodeChild<'a>>,
}

impl<'x> Node<'x> {
    pub fn is_waiting_for_node(&self, pt: &PhraseTable) -> bool {
        let phrase = pt.get(self.role).unwrap();
        assert!(phrase.components.len() > self.children.len());
        phrase.components[self.children.len()].is_node()
    }

    pub fn is_complete(&self, pt: &PhraseTable) -> bool {
        pt.get(self.role).unwrap().components.len() == self.children.len()
    }

    pub fn as_construct(
        &self,
        pt: &PhraseTable,
        env: &mut Environment<'x>,
        scope: impl Scope + 'static,
    ) -> ConstructId {
        self.as_construct_dyn_scope(pt, env, Box::new(scope))
    }

    pub fn as_construct_dyn_scope(
        &self,
        pt: &PhraseTable,
        env: &mut Environment<'x>,
        scope: Box<dyn Scope>,
    ) -> ConstructId {
        pt.get(self.role)
            .unwrap()
            .create_item
            .expect(&format!("{} is not a construct", self.role))(env, scope, self)
    }

    pub fn as_ident(&self) -> &'x str {
        if self.role != "identifier" {
            panic!("{} is not an identifier", self.role)
        }
        if self.children.len() != 2 {
            panic!("identifier is not complete")
        }
        self.children[1].as_text()
    }
}

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
        while self.0.len() >= 2 && pt[self.0[self.0.len() - 2].role].precedence <= prec {
            self.collapse(pt)
        }
    }
}
