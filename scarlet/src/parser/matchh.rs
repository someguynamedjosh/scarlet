use regex::Regex;

use super::phrase::{Phrase, PhraseTable, Precedence};
use crate::parser::stack::Stack;

pub fn anchored_find<'a>(regex: &Regex, input: &'a str) -> Option<&'a str> {
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

#[derive(Debug)]
pub enum StackAction {
    DontPopNode,
    PopNode(Precedence),
}

#[derive(Debug)]
pub struct MatchSuccess<'a> {
    pub phrase: &'static str,
    pub action: StackAction,
    pub text: &'a str,
    pub continuation_of: Option<usize>,
}

pub fn matches<'a>(
    match_against: &'a str,
    how_many_components_already_matched: usize,
    continuation_of: Option<usize>,
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
                continuation_of,
            })
        } else {
            None
        }
    } else {
        None
    }
}

pub fn longest_match<'a>(
    match_against: &'a str,
    stack: &Stack<'a>,
    phrases: &PhraseTable,
) -> Option<MatchSuccess<'a>> {
    let mut longest_match = None;
    for (node_index, node) in stack.0.iter().enumerate().rev() {
        let phrase = &phrases[node.phrase];
        if let Some(matchh) = matches(match_against, node.children.len(), Some(node_index), phrase)
        {
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
        if let Some(matchh) = matches(match_against, 0, None, phrase) {
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
