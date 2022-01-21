use typed_arena::Arena;

use super::{overlay::Overlay, Environment};
use crate::{
    constructs::{as_struct, ConstructDefinition, ConstructId},
    parser::{
        Node,
        NodeChild::{self, *},
    },
    scope::Scope,
};

#[derive(Clone, Debug)]
pub enum PathParentType {
    StructValue,
    StructRest,
}

#[derive(Clone, Debug)]
pub struct PathParent {
    typee: PathParentType,
    parent: ConstructId,
}

#[derive(Clone, Debug)]
pub struct Path {
    pub ident: String,
    pub access: Vec<PathParentType>,
}

fn text_child<'a>(code_arena: &'a Arena<String>, s: &str) -> NodeChild<'a> {
    Text(code_arena.alloc(s.to_owned()))
}

impl Path {
    pub fn vomit<'a>(&self, code_arena: &'a Arena<String>) -> Node<'a> {
        let mut result = Node {
            phrase: "identifier",
            children: vec![text_child(code_arena, &self.ident)],
        };
        for access in &self.access {
            match access {
                PathParentType::StructValue => {
                    result = Node {
                        phrase: "value access",
                        children: vec![NodeChild::Node(result), text_child(code_arena, ".VALUE")],
                    }
                }
                PathParentType::StructRest => {
                    result = Node {
                        phrase: "rest access",
                        children: vec![NodeChild::Node(result), text_child(code_arena, ".REST")],
                    }
                }
            }
        }
        result
    }
}
