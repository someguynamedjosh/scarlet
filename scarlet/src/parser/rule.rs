use regex::Regex;

use crate::parser::incoming::{
    CollapseUntilOperator, CollapseUpToPrecedence, DontCollapseStack, IncomingOperator,
    OperatorMode::*,
};

#[derive(Debug)]
pub struct Rule {
    pub(super) matchh: Regex,
    pub(super) result: IncomingOperator,
}

pub fn phrase<const LEN: usize>(
    preceding_node: Option<u8>,
    phrase: [(&str, u8, bool, Vec<Rule>); LEN],
) -> Rule {
    assert!(phrase.len() > 0);
    let mut phrase_parts = IntoIterator::into_iter(phrase);
    let (first_regex, first_precedence, first_has_next, first_extras) =
        phrase_parts.next().unwrap();
    let first_regex = Regex::new(first_regex).unwrap();
    let first_op = if let Some(prec) = preceding_node {
        IncomingOperator {
            collapse_stack_while: Box::new(CollapseUpToPrecedence(prec)),
            mode: UsePreviousAsFirstArgument,
            wait_for_next_node: first_has_next,
            precedence: first_precedence,
            extra_rules: first_extras,
        }
    } else {
        IncomingOperator {
            collapse_stack_while: Box::new(DontCollapseStack),
            mode: DontUsePrevious,
            wait_for_next_node: first_has_next,
            precedence: first_precedence,
            extra_rules: vec![],
        }
    };
    let mut result = Rule {
        matchh: first_regex.clone(),
        result: first_op,
    };
    let mut previous_extras = &mut result.result.extra_rules;
    let mut prev_regex = vec![first_regex];
    for (next_regex, next_precedence, next_has_next, next_extras) in phrase_parts {
        let next_regex = Regex::new(next_regex).unwrap();
        previous_extras.push(Rule {
            matchh: next_regex.clone(),
            result: IncomingOperator {
                collapse_stack_while: Box::new(CollapseUntilOperator(prev_regex.clone())),
                mode: AddToPrevious,
                wait_for_next_node: next_has_next,
                precedence: next_precedence,
                extra_rules: next_extras,
            },
        });
        previous_extras = &mut previous_extras[0].result.extra_rules;
        prev_regex.push(next_regex);
    }
    result
}
