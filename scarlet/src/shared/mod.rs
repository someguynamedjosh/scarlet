mod any_eq;
mod ordered_map;
mod owned_or_borrowed;
mod pool;
mod pool_id;
mod pool_traits;
mod terminal_utilities;

pub use any_eq::*;
pub use ordered_map::*;
pub use owned_or_borrowed::*;
pub use pool::*;
pub use pool_id::*;
pub use terminal_utilities::*;

pub fn indented(source: &str) -> String {
    source.replace("\n", "\n    ")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TripleBool {
    True,
    False,
    Unknown,
}
