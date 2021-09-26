//! Stage 3 dereferences items, such that the result contains no Item(id) or
//! Member{id, name} items.

mod environment_debug;
pub mod ingest;
pub mod structure;

pub use ingest::ingest;
