#![allow(incomplete_features)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(adt_const_params)]
#![feature(trait_upcasting)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

mod constructs;
mod environment;
mod file_tree;
pub mod parser;
pub mod scope;
mod shared;
mod util;
pub mod resolvable;

// use crate::{environment::Environment, scope::SRoot};

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    let root = file_tree::read_root(&path).unwrap();
    println!("{:#?}", root);

    println!("{:#?}", parser::parse(&root.self_content));

    // let mut env = Environment::new();
    // let root = transform::p_root()(&root.self_content).unwrap().1;
    // let root = env.push_unresolved(root);
    // env.set_scope(root, &SRoot);
    // env.resolve_all();
    // env.reduce_all();
    // env.check_all();
    // let root = env.resolve(root);
    // println!("{:#?}", env);
    // println!("Root: {:?}", root);
    // println!();
    // env.show_all_requested();

    // let (stage2, _s2_root) = stage2::ingest(&tokens);
    // // println!("{:#?}", stage2);
    // // println!("Root: {:?}", s2_root);
    // stage2.show_all();
}
