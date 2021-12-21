use super::stack::Node;
use crate::{
    constructs::{
        equal::CEqual,
        is_populated_struct::CIsPopulatedStruct,
        structt::{
            AtomicStructMember, CAtomicStructMember, CPopulatedStruct, SField, SFieldAndRest,
        },
        unique::CUnique,
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

pub fn is_populated_struct(
    env: &mut Environment,
    scope: Box<dyn Scope>,
    node: &Node,
) -> ConstructId {
    assert_eq!(node.arguments.len(), 1);
    let this = env.push_placeholder(scope);
    let base = node.arguments[0].as_construct(env, SPlain(this));
    env.define_construct(this, CIsPopulatedStruct::new(base));
    this
}

pub fn populated_struct(env: &mut Environment, scope: Box<dyn Scope>, node: &Node) -> ConstructId {
    assert_eq!(node.operators, &["POPULATED_STRUCT", "[", "]"]);
    assert!(node.arguments.len() == 1);
    let args = collect_comma_list(node.arguments.get(0));
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
