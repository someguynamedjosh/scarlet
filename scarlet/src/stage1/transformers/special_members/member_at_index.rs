use super::base::SpecialMember;
use crate::stage1::structure::TokenTree;

pub struct MemberAtIndex;
impl SpecialMember for MemberAtIndex {
    fn aliases(&self) -> &'static [&'static str] {
        &["MemberAtIndex", "Member", "Mem"]
    }

    fn expects_paren_group(&self) -> bool {
        true
    }

    fn apply<'t>(
        &self,
        base: TokenTree<'t>,
        paren_group: Option<Vec<TokenTree<'t>>>,
    ) -> TokenTree<'t> {
        TokenTree::BuiltinRule {
            name: "member_at_index",
            body: [vec![base], paren_group.unwrap()].concat(),
        }
    }
}
