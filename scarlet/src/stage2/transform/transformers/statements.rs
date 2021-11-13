use crate::stage2::{
    structure::Token,
    transform::{
        basics::{Transformer, TransformerResult},
        ApplyContext,
    },
};

pub struct OnPattern;
impl Transformer for OnPattern {
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        &c.to[at] == &Token::Plain("on")
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        let pattern = c.to[at + 1].clone();
        let pattern = Token::Item(c.push_token(pattern));
        let value = c.to[at + 2].clone();
        let value = Token::Item(c.push_token(value));
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
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool {
        &c.to[at] == &Token::Plain("else")
    }

    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t> {
        let value = c.to[at + 1].clone();
        let value = Token::Item(c.push_token(value));
        TransformerResult {
            replace_range: at..=at + 1,
            with: Token::Stream {
                label: "else",
                contents: vec![value],
            },
        }
    }
}
