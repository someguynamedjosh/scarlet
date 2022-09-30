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
#![feature(core_intrinsics)]
#![feature(fmt_internals)]

use std::time::Instant;

use crate::{
    environment::Environment,
    parser::{create_root, ParseContext},
};

pub mod definitions;
pub mod diagnostic;
pub mod environment;
mod file_tree;
pub mod item;
pub mod parser;
pub mod scope;
mod shared;
mod util;

fn entry() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    let time = Instant::now();
    let file_tree = file_tree::read_root(&path).unwrap();
    println!("Read source in {:#?}", time.elapsed());

    let time = Instant::now();
    let parse_context = ParseContext::new();
    let mut file_counter = 0;
    let root = parser::parse_tree(&file_tree, &parse_context, &mut file_counter);
    let root = match root {
        Ok(root) => root,
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                println!("{}", diagnostic.format_colorful(&file_tree));
            }
            return;
        }
    };
    println!("Parsed in {:#?}", time.elapsed());

    let time = Instant::now();
    let mut env = Environment::new();
    let root = create_root(&root, &parse_context, &mut env);
    let root = match root {
        Ok(root) => root,
        Err(diagnostic) => {
            println!("{}", diagnostic.format_colorful(&file_tree));
            return;
        }
    };
    println!("Created in {:#?}", time.elapsed());
    println!("{:#?}", root);
}

fn main() {
    for _ in 0..1 {
        entry();
    }
}
