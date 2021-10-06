use crate::stage1::{
    ingest::nom_prelude::*,
    structure::construct::{Construct, Position},
};

mod explicit;
mod helpers;
mod postfix_shorthands;
mod prefix_shorthands;
mod root_shorthands;

fn prefix_parser<'i>() -> impl Parser<'i, Construct> {
    alt((
        explicit::parser(Position::Prefix),
        prefix_shorthands::target_parser(),
    ))
}

fn root_parser<'i>() -> impl Parser<'i, Construct> {
    alt((
        explicit::parser(Position::Root),
        root_shorthands::integer_shorthand_parser(),
        root_shorthands::ident_parser(),
    ))
}

fn postfix_parser<'i>() -> impl Parser<'i, Construct> {
    alt((
        explicit::parser(Position::Postfix),
        postfix_shorthands::member_parser(),
        postfix_shorthands::substituting_parser(),
        postfix_shorthands::type_is_parser(),
    ))
}

impl Construct {
    pub fn parser<'i>(position: Position) -> impl Parser<'i, Self> {
        move |input| match position {
            Position::Prefix => prefix_parser()(input),
            Position::Root => root_parser()(input),
            Position::Postfix => postfix_parser()(input),
        }
    }

    pub fn type_annotation_postfix_parser<'i>() -> impl Parser<'i, Self> {
        alt((
            explicit::limited_parser(&["member", "From", "substituting"]),
            postfix_shorthands::member_parser(),
            postfix_shorthands::substituting_parser(),
        ))
    }
}
