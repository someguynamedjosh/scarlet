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
        ConstructId, shown::CShown,
    },
    environment::Environment,
    resolvable::{RSubstitution, RVariable},
    scope::{SPlain, Scope},
};

fn collect_comma_list<'a, 'n>(list: Option<&'a Node<'n>>) -> Vec<&'a Node<'n>> {
    // if let Some(list) = list {
    //     if list.operators == &[","] {
    //         assert_eq!(list.arguments.len(), 2);
    //         [
    //             collect_comma_list(Some(&list.arguments[0])),
    //             vec![&list.arguments[1]],
    //         ]
    //         .concat()
    //     } else {
    //         vec![list]
    //     }
    // } else {
    //     vec![]
    // }
    todo!()
}

pub fn atomic_struct_member<'x, const M: AtomicStructMember>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.arguments.len(), 1);
    // let this = env.push_placeholder(scope);
    // let base = node.arguments[0].as_construct(env, SPlain(this));
    // env.define_construct(this, CAtomicStructMember(base, M));
    // this
    todo!()
}

pub fn builtin_item<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &[".AS_BUILTIN_ITEM", "[", "]"]);
    // assert_eq!(node.arguments.len(), 2);
    // let base = node.arguments[0].as_construct_dyn_scope(env, scope);
    // let name = node.arguments[1].as_ident();
    // env.define_builtin_item(name, base);
    // base
    todo!()
}

pub fn equal<'x>(env: &mut Environment<'x>, scope: Box<dyn Scope>, node: &Node<'x>) -> ConstructId {
    // assert_eq!(node.operators, &["="]);
    // assert_eq!(node.arguments.len(), 2);
    // let this = env.push_placeholder(scope);
    // let left = node.arguments[0].as_construct(env, SPlain(this));
    // let right = node.arguments[1].as_construct(env, SPlain(this));
    // env.define_construct(this, CEqual::new(left, right));
    // this
    todo!()
}

pub fn if_then_else<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &["IF_THEN_ELSE", "[", "]"]);
    // assert!(node.arguments.len() == 1);
    // let args = collect_comma_list(node.arguments.get(0));
    // assert_eq!(args.len(), 3);
    // let this = env.push_placeholder(scope);

    // let condition = args[0].as_construct(env, SPlain(this));
    // let then = args[1].as_construct(env, SPlain(this));
    // let elsee = args[2].as_construct(env, SPlain(this));
    // env.define_construct(this, CIfThenElse::new(condition, then, elsee));
    // this
    todo!()
}

pub fn is_populated_struct<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &[".IS_POPULATED_STRUCT"]);
    // assert_eq!(node.arguments.len(), 1);
    // let this = env.push_placeholder(scope);
    // let base = node.arguments[0].as_construct(env, SPlain(this));
    // env.define_construct(this, CIsPopulatedStruct::new(base));
    // this
    todo!()
}

pub fn parentheses<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &["(", ")"]);
    // assert_eq!(node.arguments.len(), 1);
    // node.arguments[0].as_construct_dyn_scope(env, scope)
    todo!()
}

pub fn populated_struct<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &["POPULATED_STRUCT", "[", "]"]);
    // assert!(node.arguments.len() == 1);
    // let args = collect_comma_list(node.arguments.get(0));
    // assert_eq!(args.len(), 3);
    // let this = env.push_placeholder(scope);

    // let label = args[0].as_ident().to_owned();
    // let value = args[1].as_construct(env, SFieldAndRest(this));
    // let rest = args[2].as_construct(env, SField(this));
    // env.define_construct(this, CPopulatedStruct::new(label, value, rest));
    // this
    todo!()
}

pub fn shown<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &[".SHOWN"]);
    // assert_eq!(node.arguments.len(), 1);
    // let this = env.push_placeholder(scope);
    // let base = node.arguments[0].as_construct(env, SPlain(this));
    // env.define_construct(this, CShown::new(base));
    // this
    todo!()
}

pub fn struct_from_fields<'x>(
    env: &mut Environment<'x>,
    mut fields: Vec<(Option<&str>, &Node<'x>)>,
    scope: Box<dyn Scope>,
) -> ConstructId {
    // if fields.is_empty() {
    //     env.get_builtin_item("void")
    // } else {
    //     let (label, field) = fields.remove(0);
    //     let label = label.unwrap_or("").to_owned();
    //     let this = env.push_placeholder(scope);
    //     let field = field.as_construct(env, SFieldAndRest(this));
    //     let rest = struct_from_fields(env, fields, Box::new(SField(this)));
    //     let this_def = CPopulatedStruct::new(label, field, rest);
    //     env.define_construct(this, this_def);
    //     this
    // }
    todo!()
}

pub fn structt<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &["{", "}"]);
    // assert!(node.arguments.len() <= 1);
    // let fields = collect_comma_list(node.arguments.get(0));
    // let fields = fields
    //     .into_iter()
    //     .map(|field| {
    //         if field.operators == &["IS"] {
    //             (Some(field.arguments[0].as_ident()), &field.arguments[1])
    //         } else {
    //             (None, field)
    //         }
    //     })
    //     .collect();
    // struct_from_fields(env, fields, scope)
    todo!()
}

pub fn substitution<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &["[", "]"]);
    // assert!(node.arguments.len() <= 2);
    // let this = env.push_placeholder(scope);
    // let base = node.arguments[0].as_construct(env, SPlain(this));
    // let mut named_subs = Vec::new();
    // let mut anonymous_subs = Vec::new();
    // for sub in collect_comma_list(node.arguments.get(1)) {
    //     if sub.operators == &["IS"] {
    //         named_subs.push((
    //             sub.arguments[0].as_ident(),
    //             sub.arguments[1].as_construct(env, SPlain(this)),
    //         ));
    //     } else {
    //         anonymous_subs.push(sub.as_construct(env, SPlain(this)));
    //     }
    // }
    // env.define_unresolved(
    //     this,
    //     RSubstitution {
    //         base,
    //         named_subs,
    //         anonymous_subs,
    //     },
    // );
    // this
    todo!()
}

pub fn unique<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &["UNIQUE"]);
    // let id = env.push_unique();
    // env.push_construct(CUnique::new(id), scope)
    todo!()
}

pub fn variable<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(&node.operators[1..], &["[", "]"]);
    // assert!(node.arguments.len() <= 1);
    // let mut invariants = Vec::new();
    // let mut depends_on = Vec::new();
    // let mut mode = 0;
    // let this = env.push_placeholder(scope);
    // for arg in collect_comma_list(node.arguments.get(0)) {
    //     if arg.operators == &["IDENTIFIER", "DEPENDS_ON"] {
    //         mode = 1;
    //     } else if mode == 0 {
    //         let con = arg.as_construct(env, SVariableInvariants(this));
    //         invariants.push(con);
    //     } else {
    //         let con = arg.as_construct(env, SPlain(this));
    //         depends_on.push(con);
    //     }
    // }
    // let id = env.push_variable();
    // let def = RVariable {
    //     id,
    //     invariants,
    //     depends_on,
    // };
    // env.define_unresolved(this, def);
    // this
    todo!()
}
