#![allow(incomplete_features)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(adt_const_params)]
#![feature(trait_upcasting)]
#![feature(associated_type_defaults)]
#![feature(hash_raw_entry)]
#![feature(assert_matches)]
#![feature(ptr_to_from_bits)]
#![feature(core_intrinsics)]
#![feature(fmt_internals)]
#![feature(type_name_of_val)]

pub mod definitions;
pub mod diagnostic;
mod entry;
pub mod environment;
mod file_tree;
pub mod item;
pub mod parser;
pub mod scope;
mod shared;
mod util;

fn main() {
    for _ in 0..1 {
        entry::entry();
    }
}
