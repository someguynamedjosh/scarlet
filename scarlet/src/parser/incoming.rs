use std::fmt::{self, Debug, Formatter};

use regex::Regex;

use super::{
    rule::Rule,
    stack::{CreateFn, Node},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorMode {
    DontUsePrevious,
    UsePreviousAsFirstArgument,
    AddToPrevious,
}

pub struct IncomingOperator {
    pub(super) readable_name: &'static str,
    pub(super) create_item: Option<CreateFn>,

    pub(super) collapse_stack_while: Box<dyn StackCollapseCondition>,
    pub(super) mode: OperatorMode,
    pub(super) wait_for_next_node: bool,
    pub(super) precedence: u8,
    pub(super) extra_rules: Vec<Rule>,
}

impl Debug for IncomingOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("readable_name", &self.readable_name)
            .field("collapse_stack_while", &self.collapse_stack_while)
            .field("mode", &self.mode)
            .field("wait_for_next_node", &self.wait_for_next_node)
            .field("precedence", &self.precedence)
            .field("extra_rules", &self.extra_rules)
            .finish_non_exhaustive()
    }
}

impl IncomingOperator {
    pub fn comma() -> Self {
        Self {
            readable_name: "comma",
            create_item: None,
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
