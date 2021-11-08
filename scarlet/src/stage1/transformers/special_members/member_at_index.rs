use super::base::SpecialMember;
use crate::stage2::structure::Token;

pub struct MemberAtIndex;
impl SpecialMember for MemberAtIndex {
    fn aliases(&self) -> &'static [&'static str] {
        &["MemberAtIndex", "Member", "Mem"]
    }

    fn expects_paren_group(&self) -> bool {
        true
    }

    fn apply<'t>(&self, base: Token<'t>, paren_group: Option<Vec<Token<'t>>>) -> Token<'t> {
        Token::Stream {
            label: "member_at_index",
            contents: [vec![base], paren_group.unwrap()].concat(),
        }
    }
}
