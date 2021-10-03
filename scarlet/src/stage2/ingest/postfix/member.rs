use crate::{stage1::structure::construct::Construct, stage2::structure::Item};

pub fn ingest(base: Item, post: Construct) -> Item {
    let the_name = ingest_ident_name(post);
    Item::Member {
        base: Box::new(base),
        name: the_name,
    }
}

fn ingest_ident_name(post: Construct) -> String {
    let the_name = post
        .expect_single_expression("member")
        .expect("TODO: nice error")
        .expect_ident()
        .unwrap()
        .to_owned();
    the_name
}
