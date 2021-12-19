use std::fmt::{self, Debug, Formatter};

use regex::Regex;

pub struct Token<'a> {
    pub role: &'static str,
    pub content: &'a str,
}

impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.role, self.content)
    }
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let name = Regex::new("[a-zA-Z0-9_]+|[^a-zA-Z0-9_]").unwrap();
    let whitespace = Regex::new(r"[\r\n\t ]+").unwrap();
    let mut index = 0;
    let mut tokens = Vec::new();
    while index < input.len() {
        let (result, role) = (|| {
            if let Some(result) = whitespace.find_at(input, index) {
                if result.start() == index {
                    return (result, "whitespace");
                }
            }
            if let Some(result) = name.find_at(input, index) {
                if result.start() == index {
                    return (result, "name");
                }
            }
            panic!("Unrecognized characters in input: {}", &input[index..])
        })();
        tokens.push(Token {
            role,
            content: result.as_str(),
        });
        index = result.end();
    }
    tokens
}
