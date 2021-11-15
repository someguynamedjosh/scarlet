mod ordered_map;
mod pool;
mod pool_id;
mod pool_traits;
mod terminal_utilities;
pub mod any_eq;

pub use ordered_map::*;
pub use pool::*;
pub use pool_id::*;
pub use terminal_utilities::*;

pub fn indented(source: &str) -> String {
    source.replace("\n", "\n    ")
}
