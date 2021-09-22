mod body;
mod expect;
mod labels;

pub use body::*;
pub use labels::*;

#[derive(Clone, PartialEq)]
pub struct Construct {
    pub label: String,
    pub body: ConstructBody,
}
