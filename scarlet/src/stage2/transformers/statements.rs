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
        env: &mut Environment,
        to: &Vec<Token<'t>>,
        at: usize,
    ) -> TransformerResult<'t> {
        let pattern = to[at + 1].clone();
        let pattern = Token::Stream {
            label: "pattern",
            contents: vec![pattern],
        };
        let value = to[at + 2].clone();
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
        env: &mut Environment,
        to: &Vec<Token<'t>>,
        at: usize,
    ) -> TransformerResult<'t> {
        let value = to[at + 1].clone();
        TransformerResult {
            replace_range: at..=at + 1,
            with: Token::Stream {
                label: "else",
                contents: vec![value],
            },
        }
    }
}
