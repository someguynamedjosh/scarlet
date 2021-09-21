mod parse;
// mod simplify;
mod stage2;
mod stage3;
mod stage4;
mod util;

fn main() {
    const INPUT: &'static str = include_str!("test.rer");

    let (remainder, statements) = parse::parse(INPUT).unwrap();
    if remainder.trim().len() > 0 {
        panic!("Syntax error on {}", remainder);
    }
    let (environment, _) = stage2::ingest(statements).unwrap();
    println!("{:#?}", environment);
    let environment = stage3::ingest(&environment).unwrap();
    println!("{:#?}", environment);
    let mut environment = stage4::ingest(environment).unwrap();
    println!("{:#?}", environment);
    stage4::type_check(&environment).unwrap();
    stage4::reduce(&mut environment);
    println!("{:#?}", environment);
}
