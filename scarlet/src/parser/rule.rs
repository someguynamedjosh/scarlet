use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

use super::{rule::Component::*, token::Token};

#[derive(Clone)]
pub enum Component {
    Nonterminal(String),
    Terminal(&'static str, fn(&Token) -> bool),
}

impl Debug for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nonterminal(nt) => write!(f, "{}", nt),
            Self::Terminal(name, ..) => write!(f, "{}", name),
        }
    }
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Nonterminal(nt) => {
                if let Nonterminal(ont) = other {
                    nt == ont
                } else {
                    false
                }
            }
            Terminal(n, t) => {
                if let Terminal(on, ot) = other {
                    n == on && (*t as *const ()) == (*ot as *const ())
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Component {}

impl Hash for Component {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Nonterminal(nt) => {
                state.write_u8(0);
                nt.hash(state);
            }
            Terminal(n, t) => {
                state.write_u8(1);
                n.hash(state);
                state.write_usize((*t as *const ()) as usize);
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub produced_nonterminal: String,
    pub components: Vec<Component>,
    pub preferred: bool,
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.preferred {
            write!(f, "(low priority) ")?;
        }
        write!(f, "{} ->", self.produced_nonterminal)?;
        for component in &self.components {
            write!(f, " ")?;
            component.fmt(f)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! components {
([] [$($items:tt);*]) => {
    vec![$($items),*]
};
([:$text:tt $($input:tt)*] [$($items:tt);*]) => {
    {
        crate::components!(
            [$($input)*]
            [$($items;)* ({
                fn eval(token: &Token) -> bool {
                    quote(stringify!($text))(token)
                }
                crate::parser::rule::Component::Terminal(
                    concat!("(quote(\"", stringify!($text), "\"))"),
                    eval
                )
            })]
        )
    }
};
([$nonterminal:ident $($input:tt)*] [$($items:tt);*]) => {
    crate::components!(
        [$($input)*]
        [$($items;)* (crate::parser::rule::Component::Nonterminal(String::from(stringify!($nonterminal))))]
    )
};
([$eval:tt $($input:tt)*] [$($items:tt);*]) => {
    {
        crate::components!(
            [$($input)*]
            [$($items;)* ({
                fn eval(token: &Token) -> bool {
                    $eval(token)
                }
                crate::parser::rule::Component::Terminal(stringify!($eval), eval)
            })]
        )
    }
};
}

#[macro_export]
macro_rules! rule {
($produced_nonterminal:ident -> $($components:tt)*) => {
    Rule {
        produced_nonterminal: String::from(stringify!($produced_nonterminal)),
        components: crate::components!([$($components)*] []),
        preferred: true,
    }
};
}

#[macro_export]
macro_rules! rules {
($(($nt:ident -> $($c:tt)*))*) => {
    vec![
        $(
            rule!($nt -> $($c)*)
        ),*
    ]
}
}
