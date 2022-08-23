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

use crate::{
    environment::Environment, item::resolve::resolve_all, parser::ParseContext, scope::SRoot,
};

pub mod diagnostic;
mod environment;
mod file_tree;
mod item;
pub mod parser;
pub mod scope;
mod shared;
mod util;

fn entry() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    let file_tree = file_tree::read_root(&path).unwrap();

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
    println!("Parsed");

    let mut env = Environment::new();
    let root = root.as_item(&parse_context, &mut env, SRoot);
    let root = match root {
        Ok(root) => root,
        Err(diagnostic) => {
            println!("{}", diagnostic.format_colorful(&file_tree));
            return;
        }
    };
    if let Err(diagnostics) = resolve_all(&mut env, root.ptr_clone()) {
        for diagnostic in diagnostics {
            println!("{}", diagnostic.format_colorful(&file_tree));
        }
        return;
    }
    println!("Resolved");
    root.check_all();
    println!("Checked!");
    env.show_all_requested(&root);
}

fn main() {
    for _ in 0..1 {
        entry();
    }
}
