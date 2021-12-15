use crate::transform::{
    basics::{Parser, Precedence},
    p_identifier,
};

macro_rules! return_if_ok {
    ($e:expr) => {
        match $e {
            Ok(e) => return Ok(e),
            Err(e) => e,
        }
    };
}

pub fn p_expression<'x>(precedence: Precedence) -> impl Parser<'x> {
    move |input| match precedence {
        0 => {
            let mut error = String::new();
            error.push_str(&format!(
                "not an identifier ({})",
                return_if_ok!(p_identifier(input))
            ));
            Err(error)
        }
        _ => todo!(),
    }
}
