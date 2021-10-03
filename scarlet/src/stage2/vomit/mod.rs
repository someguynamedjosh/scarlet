use super::structure::Item;
use crate::stage1::{self};

mod helpers;
mod item;

pub use item::vomit;

pub fn completely_vomit_item(item: &Item) -> String {
    stage1::vomit(&vomit(item))
}
