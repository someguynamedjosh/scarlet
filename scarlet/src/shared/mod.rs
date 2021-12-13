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

impl TripleBool {
    pub fn and(over: Vec<TripleBool>) -> TripleBool {
        let mut known_true = true;
        for b in over {
            match b {
                TripleBool::True => (),
                TripleBool::False => return TripleBool::False,
                TripleBool::Unknown => known_true = false,
            }
        }
        if known_true {
            TripleBool::True
        } else {
            TripleBool::Unknown
        }
    }
}
