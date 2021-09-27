pub mod display;
pub mod ingest;
pub mod structure;
mod util;

pub use ingest::ingest;

pub mod find_reduction_blockers;
pub use find_reduction_blockers::find_reduction_blockers;

pub mod reduce;
pub use reduce::reduce;

pub mod type_check;
pub use type_check::type_check;
