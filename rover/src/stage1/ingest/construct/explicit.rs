use crate::stage1::{
    ingest::{construct::helpers, nom_prelude::*},
    structure::construct::{labels, Construct, ConstructBody},
};

fn label_parser<'i>(root: bool) -> impl Parser<'i, &'i str> {
    let get_label = helpers::identifier_parser();
    let resolved_label = map(get_label, labels::resolve_alias);
    
    verify(resolved_label, move |label| {
        root == labels::is_root_label(label)
    })
}

fn owned_label_parser<'i>(root: bool) -> impl Parser<'i, String> {
    map(label_parser(root), String::from)
}

fn body_parser<'i>(label: &str) -> impl Parser<'i, ConstructBody> {
    let is_text_label = labels::is_text_label(label);
    move |input| {
        if is_text_label {
            helpers::text_body_parser()(input)
        } else {
            helpers::statement_body_parser()(input)
        }
    }
}

pub fn parser<'i>(root: bool) -> impl Parser<'i, Construct> {
    move |input| {
        let (input, label) = owned_label_parser(root)(input)?;
        let (input, _) = helpers::ws_then_tag("{")(input)?;
        let (input, body) = body_parser(&label[..])(input)?;
        let (input, _) = helpers::ws_then_tag("}")(input)?;
        Ok((input, Construct { label, body }))
    }
}
