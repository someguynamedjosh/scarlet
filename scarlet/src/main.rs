#![allow(incomplete_features)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(adt_const_params)]
#![feature(trait_upcasting)]

mod file_tree;
mod shared;
mod tokens;
mod util;
mod environment;
mod constructs;

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    let root = file_tree::read_root(&path).unwrap();
    let tokens = tokens::ingest(&root);
    println!("{:#?}", tokens);

    // let (stage2, _s2_root) = stage2::ingest(&tokens);
    // // println!("{:#?}", stage2);
    // // println!("Root: {:?}", s2_root);
    // stage2.show_all();
}
