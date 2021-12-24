use std::fmt::{Debug, Formatter, self};

use crate::{environment::Environment, scope::Scope, constructs::ConstructId};

use super::phrase::PhraseTable;

#[derive(Clone)]
pub enum NodeChild<'a> {
    Node(Node<'a>),
    Text(&'a str),
    Missing,
}

impl<'a> NodeChild<'a> {
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

#[derive(Clone)]
pub struct Node<'a> {
    pub role: &'static str,
    pub children: Vec<NodeChild<'a>>,
}

impl<'a> Debug for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:#?}", self.role, self.children)
    }
}

impl<'x> Node<'x> {
    pub fn will_wait_for_text(&self, pt: &PhraseTable) -> bool {
        let phrase = pt.get(self.role).unwrap();
        for component in &phrase.components[self.children.len()..] {
            if component.is_text() {
                return true;
            }
        }
        false
    }

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