mod incoming;
mod stack;
mod token;

use self::token::Token;
use crate::parser::{
    incoming::IncomingOperator,
    stack::{Node, Stack},
};

pub fn parse(input: &str) {
    let tokens = token::tokenize(input);
    println!("{:#?}", tokens);

    let mut stack = Stack(Vec::new());

    for token in tokens {
        if let Some(op) = IncomingOperator::from_token(token) {
            stack.push_operator(token, op);
        } else if token.role == "name" {
            if let Some(true) = stack.0.last().map(|n| n.is_complete()) {
                let pretend_token = Token {
                    role: "symbol",
                    content: ",",
                };
                stack.push_operator(
                    pretend_token,
                    IncomingOperator::from_token(pretend_token).unwrap(),
                );
            }
            stack.0.push(Node::Identifier(token));
        } else if token.role == "whitespace" {
            ()
        } else {
            todo!("Nice error, not sure what to do with token {:?}", token)
        }
    }

    while stack.0.len() > 1 {
        stack.collapse();
    }

    println!("{:#?}", stack);
}
