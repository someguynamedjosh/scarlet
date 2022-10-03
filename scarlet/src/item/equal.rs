pub mod context;
pub mod trim;
pub mod equal;

use crate::shared::OrderedMap;

use super::ItemPtr;

pub type Substitutions = OrderedMap<ItemPtr, ItemPtr>;
