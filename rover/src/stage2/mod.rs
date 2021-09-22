//! Takes the AST and gives it more structure. Produces an environment with a
//! list of items. Items can refer to other items, potentially cyclically.
//! Identifiers are replaced with the items they refer to. Member references are
//! not resolved and are left as strings.

pub mod ingest;
pub mod structure;

pub use ingest::ingest;
