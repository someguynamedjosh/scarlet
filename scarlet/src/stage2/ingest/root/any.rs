use crate::{
    stage1::structure::construct::Construct,
    stage2::{self, structure::Item},
};

pub fn ingest(root: Construct) -> Item {
    let typee = Box::new(get_type_definition(root));
    Item::Any { typee }
}

fn get_type_definition(root: Construct) -> Item {
    let typee = root
        .expect_single_expression("any")
        .expect("TODO: Nice error");
    let typee = stage2::ingest(typee.clone());
    typee
}
