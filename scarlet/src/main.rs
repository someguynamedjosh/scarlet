mod entry;
mod shared;
mod stage1;
mod stage2;
mod util;

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(String::from("."));
    println!("Reading source from {}", path);

    println!("Doing stages 1 and 2");
    let (mut environment, root) = entry::start_from_root(&path).unwrap();
    println!("{:#?}", environment);
    println!("\nRESULT:\n{}", stage2::vomit_completely(&environment, root));

    // println!("Reducing everything");
    // environment.reduce_everything();
    // println!("{:#?}", environment);

    // println!("Infos:");
    // environment.display_infos();
}
