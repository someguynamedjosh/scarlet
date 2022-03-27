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

use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}};

use crate::{environment::Environment, parser::ParseContext, scope::SRoot};

mod constructs;
mod environment;
mod file_tree;
pub mod parser;
pub mod resolvable;
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
    let mut hasher = DefaultHasher::new();
    root.hash(&mut hasher);
    println!("{}", hasher.finish());
    println!("Parsed");

    let mut env = Environment::new();
    root.as_construct(&parse_context, &mut env, SRoot);
    env.resolve_all();
    println!("Resolved");
    env.check_all().unwrap();
    println!("Checked");
    env.show_all_requested();
}

fn main() {
    for _ in 0..1 {
        entry();
    }
}
