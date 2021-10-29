use crate::{
    stage1::structure::TokenTree,
    stage2::{
        ingest::top_level::IngestionContext,
        structure::{BuiltinValue, Condition, Definition},
    },
};

impl<'e, 'x> IngestionContext<'e, 'x> {
    pub fn match_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        assert_eq!(body.len(), 2);
        let base = &body[0];
        let base = self.ingest_tree(base);
        let condition_source = body[1].unwrap_builtin("patterns");
        let mut conditions = Vec::new();
        let mut else_value = None;
        for item in condition_source {
            match item {
                TokenTree::BuiltinRule { name: "on", body } => {
                    assert_eq!(body.len(), 2);
                    let pattern = body[0].unwrap_builtin("pattern");
                    assert_eq!(pattern.len(), 1);
                    let pattern = self.ingest_tree(&pattern[0]);
                    let value = self.ingest_tree(&body[1]);
                    conditions.push(Condition { pattern, value })
                }
                TokenTree::BuiltinRule { name: "else", body } => {
                    assert_eq!(body.len(), 1);
                    let value = self.ingest_tree(&body[0]);
                    else_value = Some(value);
                }
                _ => unreachable!(),
            }
        }
        let else_value = else_value.expect("TODO: Nice error, no else specified.");
        Definition::Match {
            base,
            conditions,
            else_value,
        }
    }

    pub fn matches_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        assert_eq!(body.len(), 2);
        let base = &body[0];
        let base = self.ingest_tree(base);
        let pattern = self.ingest_tree(&body[1]);

        let truee = self.begin_item(&body[1]);
        self.env.items[truee].definition = Some(Definition::BuiltinValue(BuiltinValue::Bool(true)));
        let falsee = self.begin_item(&body[1]);
        self.env.items[falsee].definition =
            Some(Definition::BuiltinValue(BuiltinValue::Bool(false)));

        let condition = Condition {
            pattern,
            value: truee,
        };
        Definition::Match {
            base,
            conditions: vec![condition],
            else_value: falsee,
        }
    }
}
