use serde::Serialize;

mod ordered_map;
mod pool;
mod pool_id;
mod pool_traits;
mod terminal_utilities;

pub use ordered_map::*;
pub use pool::*;
pub use pool_id::*;
pub use terminal_utilities::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum OpaqueClass {
    Variable,
    Variant,
}
