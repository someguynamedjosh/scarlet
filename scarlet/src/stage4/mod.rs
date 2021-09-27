pub mod display;
pub mod ingest;
pub mod structure;
mod util;

pub use ingest::ingest;

pub mod type_check;
pub use type_check::type_check;

pub mod reduce;
pub use reduce::reduce;
