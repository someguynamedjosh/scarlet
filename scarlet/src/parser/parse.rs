use regex::Regex;

use super::{
    rule::{Phrase, PhraseTable, Precedence},
    stack::NodeChild,
};
use crate::{
    constructs::ConstructId,
    environment::Environment,
    parser::{
        scarlet_rules,
        stack::{Node, Stack},
    },
    resolvable::RIdentifier,
    scope::Scope,
};

fn anchored_find<'a>(regex: &Regex, input: &'a str) -> Option<&'a str> {
    if let Some(matchh) = regex.find(input) {
        if matchh.start() == 0 {
            Some(matchh.as_str())
        } else {
            None
        }
    } else {
        None
    }
}

enum StackAction {
    DontPopNode,
    PopNode(Precedence),
}

struct MatchSuccess<'a> {
    phrase: &'static str,
    action: StackAction,
    text: &'a str,
    continuation: bool,
}

fn matches<'a>(
    match_against: &'a str,
    how_many_components_already_matched: usize,
    phrase: &Phrase,
) -> Option<MatchSuccess<'a>> {
    if let (pop_node, Some(regex)) = phrase.upcoming(how_many_components_already_matched) {
        if let Some(matchh) = anchored_find(regex, match_against) {
            let action = match pop_node {
                Some(node) => StackAction::PopNode(node.prec),
                None => StackAction::DontPopNode,
            };
            Some(MatchSuccess {
                phrase: phrase.name,
                action,
                text: matchh,
                continuation: how_many_components_already_matched != 0,
            })
        } else {
            None
        }
    } else {
        None
    }
}

fn longest_match<'a>(
    match_against: &'a str,
    stack: &Stack<'a>,
    phrases: &PhraseTable,
) -> Option<MatchSuccess<'a>> {
    let mut longest_match = None;
    for node in stack.0.iter().rev() {
        let phrase = &phrases[node.role];
        if let Some(matchh) = matches(match_against, node.children.len(), phrase) {
            if longest_match
                .as_ref()
                .map(|x: &MatchSuccess| matchh.text.len() > x.text.len())
                .unwrap_or(true)
            {
                longest_match = Some(matchh);
            }
        }
    }
    for (_, phrase) in phrases {
        if let Some(matchh) = matches(match_against, 0, phrase) {
            if longest_match
                .as_ref()
                .map(|x: &MatchSuccess| matchh.text.len() > x.text.len())
                .unwrap_or(true)
            {
                longest_match = Some(matchh);
            }
        }
    }
    longest_match
}

fn create_identifier_item<'x>(
    env: &mut Environment<'x>,
    scope: Box<dyn Scope>,
    node: &Node<'x>,
) -> ConstructId {
    assert_eq!(node.role, "identifier");
    assert_eq!(node.children.len(), 2);
    env.push_unresolved(RIdentifier(node.children[1].as_text()), scope)
}

pub struct ParseContext {
    phrases: PhraseTable,
    comma: Phrase,
}

impl ParseContext {
    pub fn new() -> Self {
        let mut phrases = PhraseTable::new();
        for phrase in scarlet_rules::rules() {
            phrases.insert(phrase.name.to_owned(), phrase);
        }
        let comma = scarlet_rules::comma();
        Self {
            phrases,
            comma,
        }
    }
}

fn push_match<'a>(pt: &PhraseTable, matchh: MatchSuccess<'a>, to: &mut Stack<'a>) {
    let mut append = Vec::new();
    if let StackAction::PopNode(prec) = matchh.action {
        to.collapse_to_precedence(pt, prec);
        append.push(NodeChild::Node(to.0.pop().unwrap()));
    }
    append.push(NodeChild::Text(matchh.text));
    if matchh.continuation {
        let index = to.0.len() - 1;
        to.0[index].children.append(&mut append);
    } else {
        to.0.push(Node {
            role: matchh.phrase,
            children: append,
        });
    }
}

pub fn parse<'a>(input: &'a str, ctx: &'a ParseContext) -> Node<'a> {
    let r_whitespace = Regex::new(r"[ \r\n\t]+|#[^\n]*").unwrap();

    let ParseContext { phrases, comma } = ctx;

    let mut stack = Stack(Vec::new());

    let mut input_position = 0;
    while input_position < input.len() {
        let match_against = &input[input_position..];
        let longest_match = longest_match(match_against, &stack, phrases);
        if let Some(matchh) = longest_match {
            if let Some(true) = stack.0.last().map(|n| n.is_complete(phrases)) {
                if let StackAction::DontPopNode = matchh.action {
                    // stack.push_operator(",", &comma);
                    todo!()
                }
            }
            input_position += matchh.text.len();
            push_match(phrases, matchh, &mut stack);
            continue;
        }
        if let Some(matchh) = anchored_find(&r_whitespace, match_against) {
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
