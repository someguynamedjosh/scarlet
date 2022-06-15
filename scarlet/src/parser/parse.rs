use regex::Regex;

use super::{
    matchh::{MatchSuccess, StackAction},
    node::{FilePosition, Node, NodeChild},
    phrase::PhraseTable,
    util,
};
use crate::{
    file_tree::FileNode,
    parser::{matchh, scarlet_phrases, stack::Stack},
};

pub struct ParseContext {
    pub(crate) phrases_sorted_by_priority: PhraseTable,
    pub(crate) phrases_sorted_by_vomit_priority: PhraseTable,
}

impl ParseContext {
    pub fn new() -> Self {
        let mut phrases_sorted_by_priority = PhraseTable::new();
        let mut source = scarlet_phrases::phrases();
        source.sort_by_key(|p| p.priority);
        for phrase in source {
            phrases_sorted_by_priority.insert(phrase.name.to_owned(), phrase);
        }

        let mut phrases_sorted_by_vomit_priority = PhraseTable::new();
        let mut source = scarlet_phrases::phrases();
        source.sort_by_key(|p| p.vomit_priority);
        for phrase in source {
            phrases_sorted_by_vomit_priority.insert(phrase.name.to_owned(), phrase);
        }

        Self {
            phrases_sorted_by_priority,
            phrases_sorted_by_vomit_priority,
        }
    }
}

fn push_match<'a>(
    pt: &PhraseTable,
    matchh: MatchSuccess<'a>,
    to: &mut Stack<'a>,
    position: FilePosition,
) {
    let mut append = Vec::new();
    if let StackAction::PopNode(prec) = matchh.action {
        to.collapse_to_precedence(pt, prec);
        if Some(to.0.len() - 1) == matchh.continuation_of {
            append.push(NodeChild::Missing);
        } else {
            let top = to.0.pop().unwrap();
            if !top.is_complete(pt) {
                panic!("Incomplete phrase {:?}", top);
            }
            append.push(NodeChild::Node(top));
        }
    } else {
        if to
            .0
            .last()
            .map(|node| node.is_complete(pt))
            .unwrap_or(false)
        {
            let matchh = MatchSuccess {
                phrase: "multiple items",
                action: StackAction::PopNode(255),
                text: ",",
                continuation_of: None,
            };
            push_match(pt, matchh, to, Default::default());
        }
    }
    append.push(NodeChild::Text(matchh.text));
    if matchh.continuation_of.is_some() {
        let index = to.0.len() - 1;
        to.0[index].children.append(&mut append);
    } else {
        to.0.push(Node {
            phrase: matchh.phrase,
            children: append,
            position,
        });
    }
}

fn parse<'a>(input: &'a str, ctx: &'a ParseContext, file_index: u32) -> Option<Node<'a>> {
    let r_whitespace = Regex::new(r"[ \r\n\t]+|#[^\n]*").unwrap();

    let ParseContext {
        phrases_sorted_by_priority: phrases,
        ..
    } = ctx;

    let mut stack = Stack(Vec::new());

    let mut input_position = 0;
    let mut comment_depth = 0;
    while input_position < input.len() {
        let match_against = &input[input_position..];
        if match_against.len() < 2 {
        } else if &match_against[0..2] == "#=" {
            comment_depth += 1;
            input_position += 2;
            continue;
        } else if &match_against[0..2] == "=#" {
            comment_depth -= 1;
            input_position += 2;
            continue;
        } else if comment_depth > 0 {
            input_position += 1;
            continue;
        }
        let longest_match = matchh::longest_match(match_against, &stack, phrases);
        if let Some(matchh) = longest_match {
            input_position += matchh.text.len();
            let start_char = input.len() - match_against.len();
            let file_position = FilePosition {
                start_char,
                end_char: start_char + matchh.text.len(),
                file_index,
            };
            push_match(phrases, matchh, &mut stack, file_position);
        } else if let Some(matchh) = matchh::anchored_find(&r_whitespace, match_against) {
            input_position += matchh.len();
        } else {
            panic!("Unrecognized input: {}", match_against);
        }
    }

    while stack.0.len() > 1 {
        stack.collapse(phrases);
    }

    stack.0.pop()
}

pub fn parse_tree<'x>(
    tree: &'x FileNode,
    ctx: &'x ParseContext,
    file_counter: &mut u32,
) -> Node<'x> {
    *file_counter += 1;
    let mut children = Vec::new();
    if tree.self_content.trim().len() > 0 {
        if let Some(content) = parse(&tree.self_content, ctx, *file_counter) {
            for child in util::collect_comma_list(&NodeChild::Node(content)) {
                children.push(child.clone());
            }
        }
    }
    for (name, child) in &tree.children {
        children.push(Node {
            phrase: "is",
            children: vec![
                NodeChild::Node(Node {
                    phrase: "identifier",
                    children: vec![NodeChild::Text(name)],
                    ..Default::default()
                }),
                NodeChild::Text("IS"),
                NodeChild::Node(parse_tree(child, ctx, file_counter)),
            ],
            ..Default::default()
        })
    }
    Node {
        phrase: "struct",
        children: vec![
            NodeChild::Text("{"),
            util::create_comma_list(children),
            NodeChild::Text("}"),
        ],
        ..Default::default()
    }
}
