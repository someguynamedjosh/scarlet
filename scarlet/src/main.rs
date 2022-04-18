#![allow(incomplete_features)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(adt_const_params)]
#![feature(trait_upcasting)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![feature(hash_raw_entry)]
#![feature(assert_matches)]
#![feature(map_first_last)]
#![feature(ptr_to_from_bits)]

use crate::{environment::Environment, parser::ParseContext, scope::SRoot, item::resolve::resolve_all};

mod item;
mod environment;
mod file_tree;
pub mod parser;
pub mod scope;
mod shared;
mod util;

fn entry() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    let root = file_tree::read_root(&path).unwrap();

    let parse_context = ParseContext::new();
    let mut file_counter = 0;
    let root = parser::parse_tree(&root, &parse_context, &mut file_counter);
    println!("Parsed");

    let mut env = Environment::new();
    let root = root.as_construct(&parse_context, &mut env, SRoot);
    resolve_all(&mut env, root.ptr_clone());
    println!("Resolved");
    root.check_all();
    env.show_all_requested(&root);
}

fn main() {
    for _ in 0..1 {
        entry();
    }
}
