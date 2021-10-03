mod entry;
mod rust_analyzer_actions;
mod shared;
mod stage1;
mod stage2;
mod stage3;
mod util;

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    println!("Doing stages 1 and 2");
    let (mut environment, root) = entry::start_from_root(&path).unwrap();
    println!("{:#?}", environment);
    println!(
        "\nRESULT:\n{}",
        stage2::completely_vomit_item(&environment, root)
    );

    println!("Doing stage 3");
    let environment = stage3::ingest(&mut environment, root);
    println!("{:#?}", environment);

    // println!("Infos:");
    // environment.display_infos();
}
