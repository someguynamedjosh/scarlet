#![feature(try_trait_v2)]

mod entry;
mod shared;
mod stage1;
mod stage2;
mod util;

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    println!("Doing stages 1 and 2");
    let environment = entry::start_from_root(&path).unwrap();
    println!("{:#?}", environment);
}
