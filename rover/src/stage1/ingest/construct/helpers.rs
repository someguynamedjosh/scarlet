pub use crate::stage1::ingest::helpers::*;
use crate::stage1::{
    ingest::nom_prelude::*,
    structure::{construct::ConstructBody, statement::Statement},
};

pub fn text_body_parser<'i>() -> impl Parser<'i, ConstructBody> {
    let ptext = take_until("}");
    map(ptext, |t| ConstructBody::PlainText(String::from(t)))
}

pub fn statement_body_parser<'i>() -> impl Parser<'i, ConstructBody> {
    let pstatements = many0(after_ws(Statement::parser()));
    map(pstatements, ConstructBody::Statements)
}
