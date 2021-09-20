mod parse;
mod simplify;
mod stage2;
mod stage3;

fn main() {
    const INPUT: &'static str = include_str!("test.rer");

    let (remainder, statements) = parse::parse(INPUT).unwrap();
    if remainder.trim().len() > 0 {
        panic!("Syntax error on {}", remainder);
    }
    let (environment, file_id) = stage2::ingest(statements).unwrap();
    println!("{:#?}", environment);
    let environment = stage3::ingest(environment).unwrap();
    println!("{:#?}", environment);
    let environment = simplify::simplify(environment);
    println!("{:#?}", environment);
}
