use super::stack::Node;
use crate::{
    constructs::{equal::CEqual, unique::CUnique, ConstructId},
    environment::Environment,
    scope::{SPlain, Scope},
};

pub fn equal(env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ConstructId {
    assert_eq!(node.operators, &["="]);
    assert_eq!(node.arguments.len(), 2);
    let this = env.push_placeholder(scope);
    let left = node.arguments[0].as_item(env, SPlain(this));
    let right = node.arguments[1].as_item(env, SPlain(this));
    env.define_construct(this, CEqual::new(left, right));
    this
}

pub fn unique(env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ConstructId {
    assert_eq!(node.operators, &["UNIQUE"]);
    let id = env.push_unique();
    env.push_construct(CUnique::new(id), scope)
}
