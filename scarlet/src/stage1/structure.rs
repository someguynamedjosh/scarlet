use std::fmt::{self, Debug, Formatter};

#[derive(Clone, PartialEq, Eq)]
pub enum Token<'i> {
    Compound {
        label: &'i str,
        body: Vec<Token<'i>>,
    },
    Symbol(&'i str),
}

impl<'i> Token<'i> {
    pub fn wrap(&mut self, compound_label: &'i str) {
        *self = Self::Compound {
            label: compound_label,
            body: vec![self.clone()],
        }
    }

    pub fn is_compound(&self, required_label: &str) -> bool {
        match self {
            Token::Compound { label, .. } => *label == required_label,
            Token::Symbol(..) => false,
        }
    }

    pub fn change_compound(&mut self, new_label: &'i str) {
        match self {
            Token::Compound { label, .. } => *label = new_label,
            Token::Symbol(..) => panic!("Not a compound"),
        }
    }

    pub fn is_symbol(&self, required_contents: &str) -> bool {
        match self {
            Token::Compound { .. } => false,
            Token::Symbol(contents) => *contents == required_contents,
        }
    }
}

impl<'i> Debug for Token<'i> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Compound { label, body } => {
                write!(f, "c.{}", label)?;
                if body.len() == 0 {
                    write!(f, "[]")
                } else if body.len() == 1 {
                    write!(f, "[{:#?}]", &body[0])
                } else {
                    f.debug_list().entries(body).finish()
                }
            }
            Self::Symbol(text) => write!(f, "s.{}", text),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Module<'a> {
    pub self_content: Vec<Token<'a>>,
    pub children: Vec<(String, Module<'a>)>,
}
