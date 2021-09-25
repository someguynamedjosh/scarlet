mod construct;
mod expression;
mod helpers;
mod nom_prelude;
mod statement;

use nom_prelude::*;

use super::structure::{expression::Expression, statement::Statement};

pub fn ingest<'i>() -> impl Parser<'i, Expression> {
    delimited(ws(), Expression::parser(), ws())
}
