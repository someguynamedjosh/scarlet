use crate::{
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item},
    },
};

pub fn ingest(env: &mut Environment, root: Construct) -> Item {
    let typee = root
        .expect_single_expression("any")
        .expect("TODO: Nice error");
    let typee = stage2::ingest_expression(env, typee.clone());
    let typee = Box::new(typee);
    let id = env.new_variable();
    Item::Any { typee, id }
}
