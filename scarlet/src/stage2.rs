pub mod ingest;
pub mod structure;
mod flatten;
mod replace;
mod dedup;
mod dependencies;
mod reduce;

pub use ingest::ingest;
