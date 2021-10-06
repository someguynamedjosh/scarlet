mod construct;
mod expression;
mod helpers;
mod nom_prelude;

use nom_prelude::*;

use super::structure::expression::Expression;

pub fn ingest<'i>() -> impl Parser<'i, Expression> {
    delimited(ws(), Expression::parser(), ws())
}
