use std::fmt::{self, Debug, Formatter};

#[derive(Clone, PartialEq, Eq)]
pub enum Token<'i> {
    Compound {
        label: &'i str,
        body: Vec<Token<'i>>,
    },
    Symbol(&'i str),
}

impl<'i> Debug for Token<'i> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Compound { label, body } => {
                write!(f, "c {}", label)?;
                f.debug_list().entries(body).finish()
            }
            Self::Symbol(text) => write!(f, "s {}", text),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Module<'a> {
    pub self_content: Vec<Token<'a>>,
    pub children: Vec<(String, Module<'a>)>,
}
