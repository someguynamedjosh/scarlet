mod incoming;
mod stack;

use regex::Regex;

use crate::parser::{
    incoming::{CollapseUpToPrecedence, IncomingOperator, OperatorMode::*},
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

struct Rule {
    matchh: Regex,
    result: IncomingOperator,
}

pub fn parse(input: &str) {
    let r_name = Regex::new(r"[a-zA-Z0-9_]+").unwrap();
    let r_whitespace = Regex::new(r"[ \r\n\t]+").unwrap();

    let rules = vec![
        Rule {
            matchh: Regex::new(r"\+").unwrap(),
            result: IncomingOperator {
                collapse_stack_while: Box::new(CollapseUpToPrecedence(5)),
                mode: UsePreviousAsFirstArgument,
                wait_for_next_node: true,
                precedence: 5,
            },
        },
        Rule {
            matchh: Regex::new(r"\^").unwrap(),
            result: IncomingOperator {
                collapse_stack_while: Box::new(CollapseUpToPrecedence(2)),
                mode: UsePreviousAsFirstArgument,
                wait_for_next_node: true,
                precedence: 3,
            },
        },
    ];

    let mut stack = Stack(Vec::new());

    let mut input_position = 0;
    while input_position < input.len() {
        let match_against = &input[input_position..];
        let (mut longest_rule, mut longest_rule_length) = (None, 0);
        for rule in &rules {
            if let Some(matchh) = anchored_find(&rule.matchh, match_against) {
                if matchh.len() > longest_rule_length {
                    longest_rule_length = matchh.len();
                    longest_rule = Some(rule);
                }
            }
        }
        if let Some(rule) = longest_rule {
            let name = &match_against[..longest_rule_length];
            stack.push_operator(name, &rule.result);
            input_position += longest_rule_length;
            continue;
        }
        if let Some(matchh) = anchored_find(&r_name, match_against) {
            if let Some(true) = stack.0.last().map(|n| n.is_complete()) {
                stack.push_operator(",", &IncomingOperator::comma());
            }

            stack.0.push(Node::Identifier(matchh));

            input_position += matchh.len();
        } else if let Some(matchh) = anchored_find(&r_whitespace, match_against) {
            input_position += matchh.len();
        }
    }

    while stack.0.len() > 1 {
        stack.collapse();
    }

    println!("{:#?}", stack);
}
