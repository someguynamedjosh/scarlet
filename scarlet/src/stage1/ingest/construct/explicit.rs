use crate::stage1::{
    ingest::{construct::helpers, nom_prelude::*},
    structure::construct::{
        labels::{self, POSTFIX_CONSTRUCT_LABELS, PREFIX_CONSTRUCT_LABELS, ROOT_CONSTRUCT_LABELS},
        Construct, ConstructBody, Position,
    },
};

fn label_parser<'i>() -> impl Parser<'i, &'i str> {
    let get_label = helpers::identifier_parser();
    map(get_label, labels::resolve_alias)
}

fn limited_label_parser<'i>(allowed_labels: &'static [&'static str]) -> impl Parser<'i, &'i str> {
    verify(label_parser(), move |label| allowed_labels.contains(label))
}

fn owned_label_parser<'i>(position: Position) -> impl Parser<'i, String> {
    let allowed = match position {
        Position::Prefix => PREFIX_CONSTRUCT_LABELS,
        Position::Root => ROOT_CONSTRUCT_LABELS,
        Position::Postfix => POSTFIX_CONSTRUCT_LABELS,
    };
    map(limited_label_parser(allowed), String::from)
}

fn body_parser<'i>(label: &str) -> impl Parser<'i, ConstructBody> {
    let is_text_label = labels::is_text_label(label);
    move |input| {
        if is_text_label {
            helpers::text_body_parser()(input)
        } else {
            helpers::expression_body_parser()(input)
        }
    }
}

pub fn parser<'i>(position: Position) -> impl Parser<'i, Construct> {
    move |input| {
        let (input, label) = owned_label_parser(position)(input)?;
        let (input, _) = helpers::ws_then_tag("{")(input)?;
        let (input, body) = body_parser(&label[..])(input)?;
        let (input, _) = helpers::ws_then_tag("}")(input)?;
        Ok((input, Construct { label, body }))
    }
}

pub fn limited_parser<'i>(allowed_labels: &'static [&'static str]) -> impl Parser<'i, Construct> {
    move |input| {
        let (input, label) = limited_label_parser(allowed_labels)(input)?;
        let label = String::from(label);
        let (input, _) = helpers::ws_then_tag("{")(input)?;
        let (input, body) = body_parser(&label[..])(input)?;
        let (input, _) = helpers::ws_then_tag("}")(input)?;
        Ok((input, Construct { label, body }))
    }
}
