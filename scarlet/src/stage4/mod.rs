use crate::stage3;

pub mod structure;
mod type_elaboration;

pub fn ingest(other: stage3::structure::Environment) -> structure::Environment {
    structure::Environment::from(other)
}
