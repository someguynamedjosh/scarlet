use super::{
    incoming::{IncomingOperator, OperatorMode},
    rule::Rule,
};

pub type CreateFn = fn();

#[derive(Debug)]
pub struct Node<'a> {
    pub readable_name: &'static str,
    pub create_item: Option<CreateFn>,

    pub operators: Vec<&'a str>,
    pub arguments: Vec<Node<'a>>,
    pub waiting: bool,
    pub precedence: u8,
    pub extra_rules: &'a [Rule],
}

impl<'a> Node<'a> {
    pub fn is_complete(&self) -> bool {
        !self.waiting
    }
}

#[derive(Debug)]
pub struct Stack<'a>(pub Vec<Node<'a>>);

impl<'a> Stack<'a> {
    pub fn collapse(&mut self) {
        assert!(self.0.len() >= 2);
        let top = self.0.pop().unwrap();
        assert!(top.is_complete());
        let next = self.0.len() - 1;
        let next = &mut self.0[next];
        assert!(next.waiting);
        next.waiting = false;
        next.arguments.push(top);
    }

    pub fn push_operator(&mut self, name: &'a str, op: &'a IncomingOperator) {
        use OperatorMode::*;

        while op.collapse_stack_while.should_collapse(&self.0[..]) {
            if self.0.len() < 2 {
                panic!(
                    "Attempted to collapse the stack too many times for {}",
                    name
                );
            }
            self.collapse();
        }
        let mut arguments = Vec::new();
        if op.mode == UsePreviousAsFirstArgument {
            arguments.push(self.0.pop().unwrap());
        }
        if op.mode == AddToPrevious {
            let node = self.0.last_mut().unwrap();
            node.operators.push(name);
            node.waiting = op.wait_for_next_node;
            node.precedence = op.precedence;
            node.extra_rules = &op.extra_rules;
        } else {
            self.0.push(Node {
                readable_name: op.readable_name,
                create_item: op.create_item,
                operators: vec![name],
                arguments,
                precedence: op.precedence,
                waiting: op.wait_for_next_node,
                extra_rules: &op.extra_rules,
            });
        }
    }
}
