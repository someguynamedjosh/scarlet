use regex::Regex;

use super::{
    matchh::{MatchSuccess, StackAction},
    node::{Node, NodeChild},
    phrase::PhraseTable,
};
use crate::parser::{matchh, scarlet_phrases, stack::Stack};

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

fn push_match<'a>(pt: &PhraseTable, matchh: MatchSuccess<'a>, to: &mut Stack<'a>) {
    let mut append = Vec::new();
    if let StackAction::PopNode(prec) = matchh.action {
        to.collapse_to_precedence(pt, prec);
        if Some(to.0.len() - 1) == matchh.continuation_of {
            append.push(NodeChild::Missing);
        } else {
            append.push(NodeChild::Node(to.0.pop().unwrap()));
        }
    } else {
        if to
            .0
            .last()
            .map(|node| node.is_complete(pt))
            .unwrap_or(false)
        {
            let matchh = MatchSuccess {
                phrase: "multiple constructs",
                action: StackAction::PopNode(255),
                text: ",",
                continuation_of: None,
            };
            push_match(pt, matchh, to);
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
        });
    }
}

pub fn parse<'a>(input: &'a str, ctx: &'a ParseContext) -> Node<'a> {
    let r_whitespace = Regex::new(r"[ \r\n\t]+|#[^\n]*").unwrap();

    let ParseContext {
        phrases_sorted_by_priority: phrases,
        ..
    } = ctx;

    let mut stack = Stack(Vec::new());

    let mut input_position = 0;
    while input_position < input.len() {
        let match_against = &input[input_position..];
        let longest_match = matchh::longest_match(match_against, &stack, phrases);
        if let Some(matchh) = longest_match {
            input_position += matchh.text.len();
            push_match(phrases, matchh, &mut stack);
        } else if let Some(matchh) = matchh::anchored_find(&r_whitespace, match_against) {
            input_position += matchh.len();
        } else {
            panic!("Unrecognized input: {}", match_against);
        }
    }

    while stack.0.len() > 1 {
        stack.collapse(phrases);
    }

    let result = stack.0.pop().unwrap();

    result
}
