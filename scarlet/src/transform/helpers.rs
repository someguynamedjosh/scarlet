use crate::tokens::structure::Token;

#[macro_export]
macro_rules! parsers {
    ($($transformer:expr),*) => {
        vec![$(Box::new($transformer) as Box<dyn crate::transform::basics::Parser>),*]
    }
}
