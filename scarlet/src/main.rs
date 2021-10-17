#![feature(try_trait_v2)]
#![feature(never_type)]

mod entry;
mod rust_analyzer_actions;
mod shared;
mod stage1;
// mod stage2;
mod util;

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    let root = entry::read_root(&path).unwrap();
    let mut stage1 = stage1::ingest(&root);
    stage1::transformers::apply_transformers(&mut stage1.self_content);
    println!("{:#?}", stage1);

    // let stage2 = stage2::ingest(&stage1.self_content);
    // println!("{:?}", stage2);
    // println!("{:#?}", stage2);
}
