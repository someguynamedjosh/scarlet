use itertools::Itertools;

use super::stack::Node;
use crate::{
    constructs::{
        self,
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

pub fn atomic_struct_member<'x, const M: AtomicStructMember>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.arguments.len(), 1);
    let this = env.push_placeholder(scope);
    let base = node.arguments[0].as_construct(env, SPlain(this));
    env.define_construct(this, CAtomicStructMember(base, M));
    this
}

pub fn builtin_item<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.operators, &[".AS_BUILTIN_ITEM", "[", "]"]);
    assert_eq!(node.arguments.len(), 2);
    let base = node.arguments[0].as_construct_dyn_scope(env, scope);
    let name = node.arguments[1].as_ident();
    env.define_builtin_item(name, base);
    base
}

pub fn equal<'x>(env: &mut Environment<'x>, scope: Box<dyn Scope>, node: &Node<'x>) -> ConstructId {
    assert_eq!(node.operators, &["="]);
    assert_eq!(node.arguments.len(), 2);
    let this = env.push_placeholder(scope);
    let left = node.arguments[0].as_construct(env, SPlain(this));
    let right = node.arguments[1].as_construct(env, SPlain(this));
    env.define_construct(this, CEqual::new(left, right));
    this
}

pub fn if_then_else<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
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

pub fn is_populated_struct<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.operators, &[".IS_POPULATED_STRUCT"]);
    assert_eq!(node.arguments.len(), 1);
    let this = env.push_placeholder(scope);
    let base = node.arguments[0].as_construct(env, SPlain(this));
    env.define_construct(this, CIsPopulatedStruct::new(base));
    this
}

pub fn parentheses<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.operators, &["(", ")"]);
    assert_eq!(node.arguments.len(), 1);
    node.arguments[0].as_construct_dyn_scope(env, scope)
}

pub fn populated_struct<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
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

pub fn struct_from_fields<'x>(
    env: &mut Environment<'x>,
    mut fields: Vec<(Option<&str>, &Node<'x>)>,
    scope: Box<dyn Scope>,
) -> ConstructId {
    if fields.is_empty() {
        env.get_builtin_item("void")
    } else {
        let (label, field) = fields.remove(0);
        let label = label.unwrap_or("").to_owned();
        let this = env.push_placeholder(scope);
        let field = field.as_construct(env, SFieldAndRest(this));
        let rest = struct_from_fields(env, fields, Box::new(SField(this)));
        let this_def = CPopulatedStruct::new(label, field, rest);
        env.define_construct(this, this_def);
        this
    }
}

pub fn structt<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.operators, &["{", "}"]);
    assert!(node.arguments.len() <= 1);
    let fields = collect_comma_list(node.arguments.get(0));
    let fields = fields
        .into_iter()
        .map(|field| {
            if field.operators == &["IS"] {
                (Some(field.arguments[0].as_ident()), &field.arguments[1])
            } else {
                (None, field)
            }
        })
        .collect();
    struct_from_fields(env, fields, scope)
}

pub fn unique<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.operators, &["UNIQUE"]);
    let id = env.push_unique();
    env.push_construct(CUnique::new(id), scope)
}

pub fn variable<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(&node.operators[1..], &["[", "]"]);
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
