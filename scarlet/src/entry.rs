
use std::time::Instant;

use crate::{
    environment::Environment,
    item::query::QueryContext,
    parser::{create_root, ParseContext, self}, file_tree,
};


/// This struct guarantees certain parts of the code remain internal to the
/// library without having to put them in the same module.
pub(crate) struct OnlyConstructedByEntry(());

pub(crate) fn entry() {
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
    let mut env = Environment::new(OnlyConstructedByEntry(()));
    let root = create_root(&root, &parse_context, &mut env);
    let root = match root {
        Ok(root) => root,
        Err(diagnostic) => {
            println!("{}", diagnostic.format_colorful(&file_tree));
            return;
        }
    };
    println!("Created in {:#?}", time.elapsed());

    let errors = env.set_root(root);
    if errors.len() > 0 {
        for error in errors {
            println!("{}", error.format_colorful(&file_tree));
        }
        return;
    }
    println!("Processed in {:#?}", time.elapsed());
}
