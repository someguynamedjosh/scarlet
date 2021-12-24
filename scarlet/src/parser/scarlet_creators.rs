use itertools::Itertools;

use self::NodeChild::*;
use super::{node::Node, ParseContext};
use crate::{
    constructs::{
        self,
        equal::CEqual,
        if_then_else::CIfThenElse,
        is_populated_struct::CIsPopulatedStruct,
        shown::CShown,
        structt::{
            AtomicStructMember, CAtomicStructMember, CPopulatedStruct, SField, SFieldAndRest,
        },
        unique::CUnique,
        variable::{CVariable, SVariableInvariants},
        ConstructId,
    },
    environment::Environment,
    parser::node::NodeChild,
    resolvable::{RIdentifier, RSubstitution, RVariable},
    scope::{SPlain, Scope},
};

fn collect_comma_list<'a, 'n>(list: &'a NodeChild<'n>) -> Vec<&'a Node<'n>> {
    if let NodeChild::Node(list) = list {
        if list.phrase == "multiple constructs" {
            assert_eq!(list.children.len(), 3);
            assert_eq!(list.children[1], NodeChild::Text(","));
            [
                collect_comma_list(&list.children[0]),
                vec![list.children[2].as_node()],
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
    pc: &ParseContext,
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
    pc: &ParseContext,
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

pub fn equal<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    // assert_eq!(node.operators, &["="]);
    // assert_eq!(node.arguments.len(), 2);
    // let this = env.push_placeholder(scope);
    // let left = node.arguments[0].as_construct(env, SPlain(this));
    // let right = node.arguments[1].as_construct(env, SPlain(this));
    // env.define_construct(this, CEqual::new(left, right));
    // this
    todo!()
}

pub fn identifier<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.phrase, "identifier");
    assert_eq!(node.children.len(), 2);
    env.push_unresolved(RIdentifier(node.children[1].as_text()), scope)
}

pub fn if_then_else<'x>(
    pc: &ParseContext,
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
    pc: &ParseContext,
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
    pc: &ParseContext,
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
    pc: &ParseContext,
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
    pc: &ParseContext,
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
    pc: &ParseContext,
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
        let field = field.as_construct(pc, env, SFieldAndRest(this));
        let rest = struct_from_fields(pc, env, fields, Box::new(SField(this)));
        let this_def = CPopulatedStruct::new(label, field, rest);
        env.define_construct(this, this_def);
        this
    }
}

pub fn structt<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0], Text("{"));
    assert_eq!(node.children[2], Text("}"));
    let fields = collect_comma_list(&node.children[1]);
    let fields = fields
        .into_iter()
        .map(|field| {
            if field.phrase == "is" {
                (
                    Some(field.children[0].as_node().children[0].as_text()),
                    field.children[2].as_node(),
                )
            } else {
                (None, field)
            }
        })
        .collect();
    struct_from_fields(pc, env, fields, scope)
}

pub fn substitution<'x>(
    pc: &ParseContext,
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
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children, &[Text("UNIQUE")]);
    let id = env.push_unique();
    env.push_construct(CUnique::new(id), scope)
}

pub fn variable<'x>(
    pc: &ParseContext,
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
