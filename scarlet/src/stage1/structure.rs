use std::fmt::Debug;

pub type Token<'i> = &'i str;

#[derive(Clone, Debug)]
pub struct Module<'a> {
    pub self_content: Vec<Token<'a>>,
    pub children: Vec<(String, Module<'a>)>,
}
