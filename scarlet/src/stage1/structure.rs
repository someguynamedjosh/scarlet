use std::fmt::Debug;

pub type Token<'i> = &'i str;

#[derive(Clone, Debug)]
pub enum TokenTree<'i> {
    Token(Token<'i>),
    Group {
        start: Token<'i>,
        end: Token<'i>,
        body: Vec<TokenTree<'i>>,
    },
    PrimitiveRule {
        name: Token<'i>,
        body: Vec<TokenTree<'i>>,
    },
}

#[derive(Clone, Debug)]
pub struct Module<'a> {
    pub self_content: Vec<TokenTree<'a>>,
    pub children: Vec<(String, Module<'a>)>,
}
