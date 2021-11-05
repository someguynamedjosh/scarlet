use crate::stage1::structure::TokenTree;

#[macro_export]
macro_rules! tfers {
    ($($transformer:expr),*) => {
        vec![$(Box::new($transformer) as Box<dyn Transformer>),*]
    }
}

pub fn expect_paren_group<'a, 't>(tt: &'a TokenTree<'t>) -> &'a Vec<TokenTree<'t>> {
    if let TokenTree::BuiltinRule {
        name: "group()",
        body,
    } = tt
    {
        body
    } else {
        todo!("nice error, expected parentheses")
    }
}
