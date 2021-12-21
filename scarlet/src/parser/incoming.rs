use std::fmt::Debug;

use regex::Regex;

use super::{rule::Rule, stack::Node};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorMode {
    DontUsePrevious,
    UsePreviousAsFirstArgument,
    AddToPrevious,
}

#[derive(Debug)]
pub struct IncomingOperator {
    pub(super) collapse_stack_while: Box<dyn StackCollapseCondition>,
    pub(super) mode: OperatorMode,
    pub(super) wait_for_next_node: bool,
    pub(super) precedence: u8,
    pub(super) extra_rules: Vec<Rule>,
}

impl IncomingOperator {
    pub fn comma() -> Self {
        Self {
            collapse_stack_while: Box::new(CollapseUpToPrecedence(254)),
            mode: OperatorMode::UsePreviousAsFirstArgument,
            wait_for_next_node: true,
            precedence: 254,
            extra_rules: vec![],
        }
    }
}

pub trait StackCollapseCondition: Debug {
    fn should_collapse(&self, stack: &[Node]) -> bool;
}

#[derive(Debug)]
pub struct DontCollapseStack;

impl StackCollapseCondition for DontCollapseStack {
    fn should_collapse(&self, _stack: &[Node]) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct CollapseUpToPrecedence(pub u8);

impl StackCollapseCondition for CollapseUpToPrecedence {
    fn should_collapse(&self, stack: &[Node]) -> bool {
        stack.len() >= 2 && stack[stack.len() - 2].precedence <= self.0
    }
}

#[derive(Debug)]
pub struct CollapseUntilOperator(pub Vec<Regex>);

impl StackCollapseCondition for CollapseUntilOperator {
    fn should_collapse(&self, stack: &[Node]) -> bool {
        let top = stack.last().unwrap();
        if top.operators.len() != self.0.len() {
            // Collapsing can continue, this is not the operator we are looking for.
            true
        } else {
            for (l, r) in top.operators.iter().zip(self.0.iter()) {
                if let Some(m) = r.find(l) {
                    if m.start() != 0 || m.end() != l.len() {
                        return true;
                    }
                } else {
                    return true;
                }
            }
            // The operator has the expected length and values, stop collapsing now.
            false
        }
    }
}
