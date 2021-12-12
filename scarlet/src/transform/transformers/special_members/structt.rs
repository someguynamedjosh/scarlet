use crate::{
    constructs::structt::{AtomicStructMember, CAtomicStructMember},
    scope::SPlain,
    tokens::structure::Token,
    transform::{transformers::special_members::base::SpecialMember, ApplyContext},
};

macro_rules! struct_access {
    ($SpecialMemberName:ident, $AtomicStructMemberName:ident, $names: expr) => {
        pub struct $SpecialMemberName;
        impl SpecialMember for $SpecialMemberName {
            fn aliases(&self) -> &'static [&'static str] {
                $names
            }

            fn apply<'t>(
                &self,
                c: &mut ApplyContext<'_, 't>,
                base: Token<'t>,
                _paren_group: Option<Vec<Token<'t>>>,
            ) -> Token<'t> {
                let base = c.env.push_unresolved(base);
                let con = CAtomicStructMember(base, AtomicStructMember::$AtomicStructMemberName);
                let con = c.env.push_construct(con);
                c.env.set_scope(base, &SPlain(con));
                Token::Construct(con)
            }

            fn vomit<'x>(
                &self,
                _c: &mut ApplyContext<'_, 'x>,
                _to: &Token<'x>,
            ) -> Option<Token<'x>> {
                None
            }
        }
    };
}

struct_access!(StructLabel, Label, &["LABEL"]);
struct_access!(StructValue, Value, &["VALUE"]);
struct_access!(StructRest, Rest, &["REST"]);
