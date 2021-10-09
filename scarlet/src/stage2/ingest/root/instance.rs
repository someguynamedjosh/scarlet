use crate::{
    shared::OpaqueClass,
    stage1::structure::construct::Construct,
    stage2::{
        self,
        structure::{Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, root: Construct) -> ItemId {
    let typee = root
        .expect_single_expression("instance_of")
        .expect("TODO: Nice error");
    let typee = stage2::ingest_expression(env, typee.clone());
    let id = env.new_opaque_value();
    let result = env.push_item(Item::Opaque {
        class: OpaqueClass::Instance,
        id,
        typee,
    });
    env.set_parent_scope(typee, result);
    result
}
