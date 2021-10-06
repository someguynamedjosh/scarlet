pub use crate::stage1::ingest::helpers::*;
use crate::stage1::{
    ingest::nom_prelude::*,
    structure::{construct::ConstructBody, expression::Expression},
};

pub fn text_body_parser<'i>() -> impl Parser<'i, ConstructBody> {
    let ptext = take_until("}");
    map(ptext, |t| ConstructBody::PlainText(String::from(t)))
}

pub fn expression_body_parser<'i>() -> impl Parser<'i, ConstructBody> {
    let pexpressions = many0(after_ws(Expression::parser()));
    map(pexpressions, ConstructBody::Expressions)
}
