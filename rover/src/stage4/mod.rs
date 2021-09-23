pub mod display;
pub mod ingest;
pub mod reduce;
pub mod structure;
pub mod type_check;
mod util;

pub use ingest::ingest;
pub use reduce::reduce;
pub use type_check::type_check;
