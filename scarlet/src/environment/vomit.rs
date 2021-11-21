use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{
        base::{BoxedConstruct, Construct},
        downcast_construct,
        shown::CShown,
    },
    tokens::structure::Token,
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

    fn vomit(&mut self, con_id: ConstructId) -> String {
        let token = self.get_construct(con_id).vomit();
        format!("{:?}", token)
    }
}
