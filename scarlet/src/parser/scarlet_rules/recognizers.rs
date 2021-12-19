use crate::parser::token::Token;

pub fn any_name(token: &Token) -> bool {
    token.role == "name"
}

pub fn any_whitespace(token: &Token) -> bool {
    token.role == "whitespace"
}

pub fn quote(text: &'static str) -> impl Fn(&Token) -> bool {
    move |token: &Token| token.content == text
}
