//! Takes in a linear sequence of characters and outputs a basic AST with most
//! syntax sugar removed. Identifiers are not resolved to what they refer to.

pub mod ingest;
pub mod structure;

pub use ingest::ingest;
