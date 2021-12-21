use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{base::Construct, downcast_construct, shown::CShown},
};

impl<'x> Environment<'x> {
    pub fn show_all_requested(&mut self) {
        let mut to_vomit = Vec::new();
        for (_, acon) in &self.constructs {
            if let ConstructDefinition::Resolved(con) = &acon.definition {
                if let Some(shown) = downcast_construct::<CShown>(&**con) {
                    to_vomit.push(shown.get_base());
                }
            }
        }
        for con_id in to_vomit {
            let con_id = self.resolve(con_id);
            let vomited = self.vomit(con_id);
            println!("{:?} is\n{}", con_id, vomited);
            println!("depends on:");
            for dep in self.get_dependencies(con_id) {
                let kind = match dep.is_capturing() {
                    true => "capturing",
                    false => "without capturing",
                };
                println!("    {} (", kind);
                for inv in dep.get_invariants() {
                    println!("        {}", self.vomit(*inv));
                }
                println!("    )");
            }
            println!();
        }
        // println!("{:#?}", self);
    }

    fn vomit(&mut self, con_id: ConstructId) -> String {
        todo!()
    }
}
