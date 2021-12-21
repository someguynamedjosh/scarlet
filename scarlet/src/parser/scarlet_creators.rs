use itertools::Itertools;

use super::stack::Node;
use crate::{
    constructs::{
        equal::CEqual,
        if_then_else::CIfThenElse,
        is_populated_struct::CIsPopulatedStruct,
        structt::{
            AtomicStructMember, CAtomicStructMember, CPopulatedStruct, SField, SFieldAndRest,
        },
        unique::CUnique,
        variable::{CVariable, SVariableInvariants},
        ConstructId,
    },
    environment::Environment,
    scope::{SPlain, Scope},
};

fn collect_comma_list<'a, 'n>(list: Option<&'a Node<'n>>) -> Vec<&'a Node<'n>> {
    if let Some(list) = list {
        if list.operators == &[","] {
            assert_eq!(list.arguments.len(), 2);
            [
                collect_comma_list(Some(&list.arguments[0])),
                vec![&list.arguments[1]],
            ]
            .concat()
        } else {
            vec![list]
        }
    } else {
        vec![]
    }
}

pub fn atomic_struct_member<const M: AtomicStructMember>(
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> ConstructId {
    assert_eq!(node.arguments.len(), 1);
    let this = env.push_placeholder(scope);
    let base = node.arguments[0].as_construct(env, SPlain(this));
    env.define_construct(this, CAtomicStructMember(base, M));
    this
}

pub fn equal(env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ConstructId {
    assert_eq!(node.operators, &["="]);
    assert_eq!(node.arguments.len(), 2);
    let this = env.push_placeholder(scope);
    let left = node.arguments[0].as_construct(env, SPlain(this));
    let right = node.arguments[1].as_construct(env, SPlain(this));
    env.define_construct(this, CEqual::new(left, right));
    this
}

pub fn if_then_else(env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ConstructId {
    assert_eq!(node.operators, &["IF_THEN_ELSE", "[", "]"]);
    assert!(node.arguments.len() == 1);
    let args = collect_comma_list(node.arguments.get(0));
    assert_eq!(args.len(), 3);
    let this = env.push_placeholder(scope);

    let condition = args[0].as_construct(env, SPlain(this));
    let then = args[1].as_construct(env, SPlain(this));
    let elsee = args[2].as_construct(env, SPlain(this));
    env.define_construct(this, CIfThenElse::new(condition, then, elsee));
    this
}

pub fn is_populated_struct(
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> ConstructId {
    assert_eq!(node.operators, &[".IS_POPULATED_STRUCT"]);
    assert_eq!(node.arguments.len(), 1);
    let this = env.push_placeholder(scope);
    let base = node.arguments[0].as_construct(env, SPlain(this));
    env.define_construct(this, CIsPopulatedStruct::new(base));
    this
}

pub fn parentheses(
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> ConstructId {
    assert_eq!(node.operators, &["(", ")"]);
    assert_eq!(node.arguments.len(), 1);
    node.arguments[0].as_construct_dyn_scope(env, scope)
}

pub fn populated_struct(env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ConstructId {
    assert_eq!(node.operators, &["POPULATED_STRUCT", "[", "]"]);
    assert!(node.arguments.len() == 1);
    let args = collect_comma_list(node.arguments.get(0));
    assert_eq!(args.len(), 3);
    let this = env.push_placeholder(scope);

    let label = args[0].as_ident().to_owned();
    let value = args[1].as_construct(env, SFieldAndRest(this));
    let rest = args[2].as_construct(env, SField(this));
    env.define_construct(this, CPopulatedStruct::new(label, value, rest));
    this
}

pub fn unique(env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ConstructId {
    assert_eq!(node.operators, &["UNIQUE"]);
    let id = env.push_unique();
    env.push_construct(CUnique::new(id), scope)
}

pub fn variable(env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ConstructId {
    assert_eq!(node.operators, &["POPULATED_STRUCT", "[", "]"]);
    assert!(node.arguments.len() <= 1);
    let invariants = collect_comma_list(node.arguments.get(0));
    let this = env.push_placeholder(scope);
    let invariants = invariants
        .into_iter()
        .map(|node| node.as_construct(env, SVariableInvariants(this)))
        .collect_vec();
    let id = env.push_variable();
    env.define_construct(this, CVariable::new(id, invariants, false, vec![]));
    this
}
