use typed_arena::Arena;

use crate::{
    constructs::{downcast_construct, if_then_else::CIfThenElse, unique::CUnique, ConstructId},
    environment::Environment,
    parser::{
        phrase::Phrase,
        util::{self, create_comma_list},
        Node, NodeChild, ParseContext,
    },
    phrase,
    scope::{SPlain, Scope},
};

fn create<'x>(
    pc: &ParseContext,
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0], NodeChild::Text("IF_THEN_ELSE"));
    assert_eq!(node.children[1], NodeChild::Text("["));
    assert_eq!(node.children[3], NodeChild::Text("]"));
    let args = util::collect_comma_list(&node.children[2]);
    assert_eq!(args.len(), 3);
    let this = env.push_placeholder(scope);

    let condition = args[0].as_construct(pc, env, SPlain(this));
    let then = args[1].as_construct(pc, env, SPlain(this));
    let elsee = args[2].as_construct(pc, env, SPlain(this));
    env.define_construct(this, CIfThenElse::new(condition, then, elsee));
    this
}

fn uncreate<'a>(
    pc: &ParseContext,
    env: &mut Environment,
    code_arena: &'a Arena<String>,
    uncreate: ConstructId,
    from: &dyn Scope,
) -> Option<Node<'a>> {
    if let Some(cite) = downcast_construct::<CIfThenElse>(&**env.get_construct_definition(uncreate))
    {
        let cite = cite.clone();
        Some(Node {
            phrase: "if then else",
            children: vec![
                NodeChild::Text("IF_THEN_ELSE"),
                NodeChild::Text("["),
                create_comma_list(vec![
                    env.vomit(255, true, pc, code_arena, cite.condition(), from),
                    env.vomit(255, true, pc, code_arena, cite.then(), from),
                    env.vomit(255, true, pc, code_arena, cite.elsee(), from),
                ]),
                NodeChild::Text("]"),
            ],
        })
    } else {
        None
    }
}

fn vomit(pc: &ParseContext, src: &Node) -> String {
    format!("IF_THEN_ELSE[ {} ]", src.children[2].as_node().vomit(pc))
}

pub fn phrase() -> Phrase {
    phrase!(
        "if then else",
        Some((create, uncreate)),
        vomit,
        0 => r"\bIF_THEN_ELSE\b" , r"\[", 255, r"\]"
    )
}
