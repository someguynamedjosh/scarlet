use super::structure::{Environment, Item};
use crate::stage1::{self};

mod helpers;
mod item;
mod value;

pub use item::vomit;

pub fn completely_vomit_item(env: &Environment, item: Item) -> String {
    stage1::vomit(&vomit(env, item))
}
