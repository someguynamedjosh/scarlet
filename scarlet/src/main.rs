#![allow(incomplete_features)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(adt_const_params)]
#![feature(trait_upcasting)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

use crate::{environment::Environment, parser::ParseContext, scope::SRoot};

mod constructs;
mod environment;
mod file_tree;
pub mod parser;
pub mod resolvable;
pub mod scope;
mod shared;
mod util;

// use crate::{environment::Environment, scope::SRoot};

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    let root = file_tree::read_root(&path).unwrap();
    println!("{:#?}", root);

    let parse_context = ParseContext::new();
    let root = parser::parse(&root.self_content, &parse_context);
    println!("{:#?}", root);

    let mut env = Environment::new();
    let root = root.as_construct(&parse_context, &mut env, SRoot);
    println!("{:#?}", env);
    println!("Root: {:?}", root);
    env.resolve_all();
    env.reduce_all();
    // env.check_all();
    // println!("{:#?}", env);
    println!("Root: {:?}", root);
    println!();
    env.show_all_requested();
}
