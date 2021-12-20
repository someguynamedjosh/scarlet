use regex::Regex;

use super::{incoming::IncomingOperator, rule::Rule};
use crate::parser::{
    scarlet_rules,
    stack::{Node, Stack},
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

fn match_longest_rule<'a>(
    match_against: &str,
    stack: &Stack<'a>,
    rules: &'a [Rule],
) -> (Option<&'a Rule>, usize) {
    let (mut longest_rule, mut longest_rule_length) = (None, 0);
    for frame in &stack.0 {
        if let Node::Operator { extra_rules, .. } = frame {
            for rule in *extra_rules {
                if let Some(matchh) = anchored_find(&rule.matchh, match_against) {
                    if matchh.len() > longest_rule_length {
                        longest_rule_length = matchh.len();
                        longest_rule = Some(rule);
                    }
                }
            }
        }
    }
    for rule in rules {
        if let Some(matchh) = anchored_find(&rule.matchh, match_against) {
            if matchh.len() > longest_rule_length {
                longest_rule_length = matchh.len();
                longest_rule = Some(rule);
            }
        }
    }
    (longest_rule, longest_rule_length)
}

pub fn parse(input: &str) {
    let r_name = Regex::new(r"[a-zA-Z0-9_]+").unwrap();
    let r_whitespace = Regex::new(r"[ \r\n\t]+").unwrap();

    let rules = scarlet_rules::rules();
    let comma = IncomingOperator::comma();

    let mut stack = Stack(Vec::new());

    let mut input_position = 0;
    while input_position < input.len() {
        let match_against = &input[input_position..];
        let (longest_rule, longest_rule_length) =
            match_longest_rule(match_against, &stack, &rules[..]);
        if let Some(rule) = longest_rule {
            let name = &match_against[..longest_rule_length];
            stack.push_operator(name, &rule.result);
            input_position += longest_rule_length;
            continue;
        }
        if let Some(matchh) = anchored_find(&r_name, match_against) {
            if let Some(true) = stack.0.last().map(|n| n.is_complete()) {
                stack.push_operator(",", &comma);
            }

            stack.0.push(Node::Identifier(matchh));

            input_position += matchh.len();
        } else if let Some(matchh) = anchored_find(&r_whitespace, match_against) {
            input_position += matchh.len();
        } else {
            panic!("Unrecognized input: {}", match_against);
        }
    }

    while stack.0.len() > 1 {
        stack.collapse();
    }

    let result = stack.0.pop().unwrap();

    println!("{:#?}", result);
}
