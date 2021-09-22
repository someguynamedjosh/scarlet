mod construct;
mod expression;
mod helpers;
mod nom_prelude;
mod statement;

use nom_prelude::*;

use super::structure::statement::Statement;

pub fn ingest<'i>() -> impl Parser<'i, Vec<Statement>> {
    many0(after_ws(Statement::parser()))
}
