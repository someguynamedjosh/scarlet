use super::incoming::{IncomingOperator, OperatorMode};

#[derive(Debug)]
pub struct Stack<'a>(pub Vec<Node<'a>>);

#[derive(Debug)]
pub enum Node<'a> {
    Operator {
        operators: Vec<&'a str>,
        arguments: Vec<Node<'a>>,
        waiting: bool,
        precedence: u8,
    },
    Identifier(&'a str),
}

impl<'a> Node<'a> {
    pub fn is_complete(&self) -> bool {
        match self {
            &Node::Operator { waiting, .. } => !waiting,
            Node::Identifier(_) => true,
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            &Node::Operator { precedence, .. } => precedence,
            Node::Identifier(_) => 0,
        }
    }
}

impl<'a> Stack<'a> {
    pub fn collapse(&mut self) {
        assert!(self.0.len() >= 2);
        let top = self.0.pop().unwrap();
        assert!(top.is_complete());
        let next = self.0.len() - 1;
        let next = &mut self.0[next];
        if let Node::Operator {
            arguments, waiting, ..
        } = next
        {
            assert!(*waiting);
            *waiting = false;
            arguments.push(top);
        } else {
            todo!("Nice error, cannot collapse self.")
        }
    }

    pub fn push_operator(&mut self, name: &'a str, op: &IncomingOperator) {
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
            if let Node::Operator {
                operators,
                precedence,
                waiting,
                ..
            } = self.0.last_mut().unwrap()
            {
                operators.push(name);
                *waiting = op.wait_for_next_node;
                *precedence = op.precedence;
            } else {
                panic!("Looks like someone didn't write their stack collapsing condition correctly")
            }
        } else {
            self.0.push(Node::Operator {
                operators: vec![name],
                arguments,
                precedence: op.precedence,
                waiting: op.wait_for_next_node,
            });
        }
    }
}
