mod ast;
mod token;

use OperatorMode::*;

use self::token::Token;

#[derive(Clone, Debug)]
enum Node<'a> {
    Operator {
        operators: Vec<Token<'a>>,
        arguments: Vec<Node<'a>>,
        waiting: bool,
        precedence: u8,
    },
    Identifier(Token<'a>),
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OperatorMode {
    DontUsePrevious,
    UsePreviousAsFirstArgument,
    AddToPrevious,
}

struct IncomingOperator {
    collapse_stack_while: Box<dyn StackCollapseCondition>,
    mode: OperatorMode,
    wait_for_next_node: bool,
    precedence: u8,
}

impl IncomingOperator {
    pub fn from_token(token: Token) -> Option<Self> {
        Some(match token.content {
            "(" => IncomingOperator {
                collapse_stack_while: Box::new(DontCollapseStack),
                mode: DontUsePrevious,
                wait_for_next_node: true,
                precedence: 255,
            },
            ")" => IncomingOperator {
                collapse_stack_while: Box::new(CollapseUntilOperator(&["("])),
                mode: AddToPrevious,
                wait_for_next_node: false,
                precedence: 0,
            },
            "," => IncomingOperator {
                collapse_stack_while: Box::new(CollapseUpToPrecedence(254)),
                mode: UsePreviousAsFirstArgument,
                wait_for_next_node: true,
                precedence: 254,
            },
            "+" => IncomingOperator {
                collapse_stack_while: Box::new(CollapseUpToPrecedence(5)),
                mode: UsePreviousAsFirstArgument,
                wait_for_next_node: true,
                precedence: 5,
            },
            "*" => IncomingOperator {
                collapse_stack_while: Box::new(CollapseUpToPrecedence(4)),
                mode: UsePreviousAsFirstArgument,
                wait_for_next_node: true,
                precedence: 4,
            },
            _ => return None,
        })
    }
}

trait StackCollapseCondition {
    fn should_collapse(&self, stack: &[Node]) -> bool;
}

pub struct DontCollapseStack;

impl StackCollapseCondition for DontCollapseStack {
    fn should_collapse(&self, _stack: &[Node]) -> bool {
        false
    }
}

pub struct CollapseUpToPrecedence(u8);

impl StackCollapseCondition for CollapseUpToPrecedence {
    fn should_collapse(&self, stack: &[Node]) -> bool {
        stack.len() >= 2 && stack[stack.len() - 2].precedence() <= self.0
    }
}

pub struct CollapseUntilOperator(&'static [&'static str]);

impl StackCollapseCondition for CollapseUntilOperator {
    fn should_collapse(&self, stack: &[Node]) -> bool {
        if let Node::Operator { operators, .. } = stack.last().unwrap() {
            if operators.len() != self.0.len() {
                // Collapsing can continue, this is not the operator we are looking for.
                true
            } else {
                for (l, r) in operators.iter().zip(self.0.iter()) {
                    if l.content != *r {
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

pub fn parse(input: &str) {
    let tokens = token::tokenize(input);
    println!("{:#?}", tokens);

    let mut stack = Vec::new();

    fn collapse(stack: &mut Vec<Node>) {
        assert!(stack.len() >= 2);
        let top = stack.pop().unwrap();
        assert!(top.is_complete());
        let next = stack.len() - 1;
        let next = &mut stack[next];
        if let Node::Operator {
            arguments, waiting, ..
        } = next
        {
            assert!(*waiting);
            *waiting = false;
            arguments.push(top);
        } else {
            todo!("Nice error, cannot collapse stack.")
        }
    }

    fn push_operator<'a>(token: Token<'a>, op: IncomingOperator, stack: &mut Vec<Node<'a>>) {
        while op.collapse_stack_while.should_collapse(&stack[..]) {
            if stack.len() < 2 {
                panic!(
                    "Attempted to collapse the stack too many times for {}",
                    token.content
                );
            }
            collapse(stack);
        }
        let mut arguments = Vec::new();
        if op.mode == UsePreviousAsFirstArgument {
            arguments.push(stack.pop().unwrap());
        }
        if op.mode == AddToPrevious {
            if let Node::Operator {
                operators,
                precedence,
                waiting,
                ..
            } = stack.last_mut().unwrap()
            {
                operators.push(token);
                *waiting = op.wait_for_next_node;
                *precedence = op.precedence;
            } else {
                panic!("Looks like someone didn't write their stack collapsing condition correctly")
            }
        } else {
            stack.push(Node::Operator {
                operators: vec![token],
                arguments,
                precedence: op.precedence,
                waiting: op.wait_for_next_node,
            });
        }
    }

    for token in tokens {
        if let Some(op) = IncomingOperator::from_token(token) {
            push_operator(token, op, &mut stack);
        } else if token.role == "name" {
            if let Some(true) = stack.last().map(|n| n.is_complete()) {
                let pretend_token = Token {
                    role: "symbol",
                    content: ",",
                };
                push_operator(
                    pretend_token,
                    IncomingOperator::from_token(pretend_token).unwrap(),
                    &mut stack,
                );
            }
            stack.push(Node::Identifier(token));
        } else if token.role == "whitespace" {
            ()
        } else {
            todo!("Nice error, not sure what to do with token {:?}", token)
        }
        println!("{:#?}", stack);
    }

    while stack.len() > 1 {
        collapse(&mut stack);
    }

    println!("{:#?}", stack);
}
