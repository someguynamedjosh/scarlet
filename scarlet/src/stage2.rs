pub mod ingest;
pub mod structure;
mod flatten;
mod replace;
mod dedup;
mod dependencies;
mod reduce;
mod vomit;

pub use ingest::ingest;
