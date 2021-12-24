use std::fmt::{self, Debug, Formatter};

use super::{phrase::PhraseTable, ParseContext};
use crate::{constructs::ConstructId, environment::Environment, scope::Scope};

#[derive(Clone, PartialEq, Eq)]
pub enum NodeChild<'a> {
    Node(Node<'a>),
    Text(&'a str),
    Missing,
}

impl<'a> NodeChild<'a> {
    pub fn as_node(&self) -> &Node<'a> {
        if let Self::Node(v) = self {
            v
        } else {
            panic!("Expected node")
        }
    }

    pub fn as_text(&self) -> &'a str {
        match self {
            NodeChild::Node(_) => panic!("Expected text, got a node instead"),
            NodeChild::Text(text) => text,
            NodeChild::Missing => panic!("Expected text, got missing instead"),
        }
    }
}

impl<'a> Debug for NodeChild<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NodeChild::Node(node) => node.fmt(f),
            NodeChild::Text(text) => text.fmt(f),
            NodeChild::Missing => write!(f, "missing"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Node<'a> {
    pub phrase: &'static str,
    pub children: Vec<NodeChild<'a>>,
}

impl<'a> Debug for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:#?}", self.phrase, self.children)
    }
}

impl<'x> Node<'x> {
    pub fn will_wait_for_text(&self, pt: &PhraseTable) -> bool {
        let phrase = pt.get(self.phrase).unwrap();
        for component in &phrase.components[self.children.len()..] {
            if component.is_text() {
                return true;
            }
        }
        false
    }

    pub fn is_waiting_for_node(&self, pt: &PhraseTable) -> bool {
        let phrase = pt.get(self.phrase).unwrap();
        assert!(phrase.components.len() > self.children.len());
        phrase.components[self.children.len()].is_node()
    }

    pub fn is_complete(&self, pt: &PhraseTable) -> bool {
        pt.get(self.phrase).unwrap().components.len() == self.children.len()
    }

    pub fn as_construct(
        &self,
        pc: &ParseContext,
        env: &mut Environment<'x>,
        scope: impl Scope + 'static,
    ) -> ConstructId {
        self.as_construct_dyn_scope(pc, env, Box::new(scope))
    }

    pub fn as_construct_dyn_scope(
        &self,
        pc: &ParseContext,
        env: &mut Environment<'x>,
        scope: Box<dyn Scope>,
    ) -> ConstructId {
        pc.phrases
            .get(self.phrase)
            .unwrap()
            .create_item
            .expect(&format!("{} is not a construct", self.phrase))(pc, env, scope, self)
    }

    pub fn as_ident(&self) -> &'x str {
        if self.phrase != "identifier" {
            panic!("{} is not an identifier", self.phrase)
        }
        if self.children.len() != 1 {
            panic!("identifier is not complete")
        }
        self.children[0].as_text()
    }
}
