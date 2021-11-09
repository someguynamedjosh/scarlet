use crate::stage2::{
    structure::{Environment, Token},
    transformers::basics::{Transformer, TransformerResult},
};

pub struct OnPattern;
impl Transformer for OnPattern {
    fn should_be_applied_at(&self, to: &[Token], at: usize) -> bool {
        &to[at] == &Token::Plain("on")
    }

    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        to: &Vec<Token<'t>>,
        at: usize,
    ) -> TransformerResult<'t> {
        let pattern = to[at + 1].clone();
        let pattern = Token::Item(env.push_token(pattern));
        let value = to[at + 2].clone();
        let value = Token::Item(env.push_token(value));
        TransformerResult {
            replace_range: at..=at + 2,
            with: Token::Stream {
                label: "on",
                contents: vec![pattern, value],
            },
        }
    }
}

pub struct Else;
impl Transformer for Else {
    fn should_be_applied_at(&self, to: &[Token], at: usize) -> bool {
        &to[at] == &Token::Plain("else")
    }

    fn apply<'t>(
        &self,
        env: &mut Environment<'t>,
        to: &Vec<Token<'t>>,
        at: usize,
    ) -> TransformerResult<'t> {
        let value = to[at + 1].clone();
        let value = Token::Item(env.push_token(value));
        TransformerResult {
            replace_range: at..=at + 1,
            with: Token::Stream {
                label: "else",
                contents: vec![value],
            },
        }
    }
}
