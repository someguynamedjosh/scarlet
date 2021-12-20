use super::Node;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorMode {
    DontUsePrevious,
    UsePreviousAsFirstArgument,
    AddToPrevious,
}

pub struct IncomingOperator {
    pub(super) collapse_stack_while: Box<dyn StackCollapseCondition>,
    pub(super) mode: OperatorMode,
    pub(super) wait_for_next_node: bool,
    pub(super) precedence: u8,
}

impl IncomingOperator {
    pub fn comma() -> Self {
        Self {
            collapse_stack_while: Box::new(CollapseUpToPrecedence(254)),
            mode: OperatorMode::UsePreviousAsFirstArgument,
            wait_for_next_node: true,
            precedence: 254,
        }
    }
}

pub trait StackCollapseCondition {
    fn should_collapse(&self, stack: &[Node]) -> bool;
}

pub struct DontCollapseStack;

impl StackCollapseCondition for DontCollapseStack {
    fn should_collapse(&self, _stack: &[Node]) -> bool {
        false
    }
}

pub struct CollapseUpToPrecedence(pub u8);

impl StackCollapseCondition for CollapseUpToPrecedence {
    fn should_collapse(&self, stack: &[Node]) -> bool {
        stack.len() >= 2 && stack[stack.len() - 2].precedence() <= self.0
    }
}

pub struct CollapseUntilOperator(pub &'static [&'static str]);

impl StackCollapseCondition for CollapseUntilOperator {
    fn should_collapse(&self, stack: &[Node]) -> bool {
        if let Node::Operator { operators, .. } = stack.last().unwrap() {
            if operators.len() != self.0.len() {
                // Collapsing can continue, this is not the operator we are looking for.
                true
            } else {
                for (l, r) in operators.iter().zip(self.0.iter()) {
                    if l != r {
                        return true;
                    }
                }
                // The operator has the expected length and values, stop collapsing now.
                false
            }
        } else {
            true
        }
    }
}
