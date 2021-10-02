use super::structure::{Environment, Item, Namespace, Value, ValueId};
use crate::{
    stage1::{
        self,
        structure::{
            construct::{Construct, ConstructBody},
            expression::Expression,
            statement::{Is, Replace, Statement},
        },
    },
    stage2::structure::{BuiltinOperation, BuiltinValue},
};

mod helpers;
mod item;
mod value;

pub use item::vomit;

pub fn completely_vomit_item(env: &Environment, item: Item) -> String {
    stage1::vomit(&vomit(env, item))
}

