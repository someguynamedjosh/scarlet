mod basics;
mod helpers;
mod pattern;
mod combinators;
mod transformers;

pub use basics::*;

pub fn p_root<'x>() -> impl Parser<'x> {
    transformers::expression::p_expression(Precedence::MAX)
}
