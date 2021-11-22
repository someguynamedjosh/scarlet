use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{base::Construct, downcast_construct, shown::CShown},
    tokens::structure::Token,
    transform::{self, ApplyContext},
};

impl<'x> Environment<'x> {
    pub fn show_all_requested(&mut self) {
        let mut to_vomit = Vec::new();
        for (_, acon) in &self.constructs {
            if let ConstructDefinition::Resolved(con) = &acon.definition {
                if let Some(shown) = downcast_construct::<CShown>(&**con) {
                    to_vomit.push(shown.0);
                }
            }
        }
        for con_id in to_vomit {
            // let con_id = self.resolve(con_id);
            let con_id = self.reduce(con_id);
            let vomited = self.vomit(con_id);
            println!("{:?} is\n{}", con_id, vomited);
        }
    }

    fn expand_stream(&mut self, input: Vec<Token<'x>>) -> Vec<Token<'x>> {
        let mut result = Vec::new();
        let extras = Default::default();
        let tfers = transform::all_transformers(&extras);
        'tokens: for token in input {
            for tfer in &tfers {
                let mut context = ApplyContext {
                    env: self,
                    parent_scope: None,
                };
                if let Some(replace_with) = tfer.as_ref().vomit(&mut context, &token) {
                    result.append(&mut self.expand_stream(replace_with));
                    continue 'tokens;
                }
            }
            result.push(token);
        }
        result
    }

    fn vomit(&mut self, con_id: ConstructId) -> String {
        let mut result = String::new();
        for token in self.expand_stream(vec![con_id.into()]) {
            result.push_str(&format!("{:?}", token));
        }
        result
    }
}
