use super::structure::{Environment, ItemId};
use crate::stage1::{self};

mod helpers;
mod item;

pub use item::vomit;

pub fn completely_vomit_item(env: &Environment, item: ItemId) -> String {
    stage1::vomit(&vomit(env, item))
}
