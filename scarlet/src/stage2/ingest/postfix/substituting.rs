use crate::{
    stage1::structure::{construct::Construct, statement::Statement},
    stage2::{
        self,
        structure::{Environment, Item, ItemId},
    },
};

pub fn ingest(env: &mut Environment, base: ItemId, post: Construct) -> ItemId {
    let substitution = post.expect_single_statement("substituting").unwrap();
    if let Statement::Is(is) = substitution {
        let target = stage2::ingest_expression(env, is.name.clone());
        let value = stage2::ingest_expression(env, is.value.clone());
        env.push_item(Item::Substituting {
            base,
            target,
            value,
        })
    } else {
        todo!("nice error")
    }
}
