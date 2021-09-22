use nom::IResult;

use self::{
    nom_prelude::{after_ws, many0, ws},
    statements::Statement,
};

pub mod construct;
pub mod expression;
pub mod nom_prelude;
pub mod statements;

pub fn parse(input: &str) -> IResult<&str, Vec<Statement>> {
    let (input, res) = many0(after_ws(Statement::parser()))(input)?;
    let (input, _) = ws()(input)?;
    Ok((input, res))
}

fn indented(source: &str) -> String { format!("    {}", source.replace("\n", "\n    ")) }
