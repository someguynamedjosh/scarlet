use self::token::Token;

mod ast;
mod token;

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

struct IncomingOperator {
    collapse_stack_while: Option<fn(&[Node]) -> bool>,
    uses_previous_node: bool,
    wait_for_next_node: bool,
    precedence: u8,
}

impl IncomingOperator {
    pub fn from_token(token: Token) -> Option<Self> {
        Some(match token.content {
            "+" => IncomingOperator {
                collapse_stack_while: Some(collapse_up_to_precedence::<5>),
                uses_previous_node: true,
                wait_for_next_node: true,
                precedence: 5,
            },
            "*" => IncomingOperator {
                collapse_stack_while: Some(collapse_up_to_precedence::<4>),
                uses_previous_node: true,
                wait_for_next_node: true,
                precedence: 4,
            },
            _ => return None,
        })
    }
}

fn collapse_up_to_precedence<const PREC: u8>(stack: &[Node]) -> bool {
    stack.len() >= 2 && stack[stack.len() - 2].precedence() <= PREC
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
        if let Some(collapse_stack_while) = op.collapse_stack_while {
            while collapse_stack_while(&stack[..]) {
                collapse(stack);
            }
        }
        let mut arguments = Vec::new();
        if op.uses_previous_node {
            arguments.push(stack.pop().unwrap());
        }
        stack.push(Node::Operator {
            operators: vec![token],
            arguments,
            precedence: op.precedence,
            waiting: op.wait_for_next_node,
        });
    }

    for token in tokens {
        if let Some(op) = IncomingOperator::from_token(token) {
            push_operator(token, op, &mut stack);
        } else if token.role == "name" {
            stack.push(Node::Identifier(token));
        } else if token.role == "whitespace" {
            ()
        } else {
            todo!("Nice error, not sure what to do with token {:?}", token)
        }
    }

    while stack.len() > 1 {
        collapse(&mut stack);
    }

    println!("{:#?}", stack);
}
