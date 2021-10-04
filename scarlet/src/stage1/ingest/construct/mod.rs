use crate::stage1::{ingest::nom_prelude::*, structure::construct::Construct};

mod explicit;
mod helpers;
mod postfix_shorthands;
mod root_shorthands;

fn root_parser<'i>() -> impl Parser<'i, Construct> {
    alt((
        explicit::parser(true),
        root_shorthands::integer_shorthand_parser(),
        root_shorthands::ident_parser(),
    ))
}

fn postfix_parser<'i>() -> impl Parser<'i, Construct> {
    alt((
        explicit::parser(false),
        postfix_shorthands::member_parser(),
        postfix_shorthands::substituting_parser(),
        postfix_shorthands::type_is_parser(),
    ))
}

impl Construct {
    pub fn parser<'i>(root: bool) -> impl Parser<'i, Self> {
        move |input| {
            if root {
                root_parser()(input)
            } else {
                postfix_parser()(input)
            }
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
