mod ast;
mod rule;
mod scarlet_rules;
mod state;
mod state_set;
mod token;
mod top_level;

pub fn parse(input: &str) {
    let rules = scarlet_rules::scarlet_rules();
    println!("{:#?}", rules);
    let tokens = token::tokenize(input);
    let ast = top_level::parse_to_ast(&tokens[..], &rules[..], "Root");
    // println!("{:#?}", ast.unwrap());
}
