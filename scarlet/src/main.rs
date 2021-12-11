#![allow(incomplete_features)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(adt_const_params)]
#![feature(trait_upcasting)]

mod constructs;
mod environment;
mod file_tree;
mod shared;
mod tokens;
pub mod transform;
mod util;
pub mod scope;

use crate::environment::Environment;

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    let root = file_tree::read_root(&path).unwrap();
    let root = tokens::ingest(&root);
    println!("{:#?}", root);

    let mut env = Environment::new();
    let scope = env.root_scope();
    let root = env.push_unresolved(root.self_content.clone(), scope);
    let root = env.resolve(root);
    env.reduce_all();
    let root = env.resolve(root);
    println!("{:#?}", env);
    println!("Root: {:?}", root);
    println!();
    env.show_all_requested();

    // let (stage2, _s2_root) = stage2::ingest(&tokens);
    // // println!("{:#?}", stage2);
    // // println!("Root: {:?}", s2_root);
    // stage2.show_all();
}
