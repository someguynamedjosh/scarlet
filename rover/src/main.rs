mod stage1;
mod stage2;
mod stage3;
mod stage4;
mod util;

fn main() {
    const INPUT: &'static str = include_str!("test.rer");

    let (remainder, statements) = stage1::ingest()(INPUT).unwrap();
    if remainder.trim().len() > 0 {
        panic!("Syntax error on {}", remainder);
    }
    println!("{:#?}", statements);
    println!("Doing stage 2");
    let (environment, _) = stage2::ingest(statements).unwrap();
    println!("{:#?}", environment);
    println!("Doing stage 3");
    let environment = stage3::ingest(&environment).unwrap();
    println!("{:#?}", environment);
    println!("Doing stage 4");
    let mut environment = stage4::ingest(environment).unwrap();
    println!("{:#?}", environment);
    println!("Doing type check");
    stage4::type_check(&environment).unwrap();
    println!("Doing reduce");
    stage4::reduce(&mut environment);
    println!("{:#?}", environment);
}
