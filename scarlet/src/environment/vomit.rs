use super::{Construct, ConstructId, Environment, path::PathOverlay};
use crate::constructs::{downcast_construct, shown::CShown, ConstructDefinition};

impl<'x> Environment<'x> {
    pub fn show_all_requested(&mut self) {
        let mut to_vomit = Vec::new();
        for (from, acon) in &self.constructs {
            if let ConstructDefinition::Resolved(con) = &acon.definition {
                if let Some(shown) = downcast_construct::<CShown>(&**con) {
                    to_vomit.push((shown.get_base(), from));
                }
            }
        }
        for (con_id, from) in to_vomit {
            let vomited = self.vomit(con_id, from);
            println!("{:?} is\n{}", con_id, vomited);
            println!("depends on:");
            for dep in self.get_dependencies(con_id) {
                let kind = match dep.is_capturing() {
                    true => "capturing",
                    false => "without capturing",
                };
                println!("    {} (", kind);
                for inv in dep.get_invariants() {
                    println!("        {}", self.vomit(*inv, from));
                }
                println!("    )");
            }
            println!();
        }
        // println!("{:#?}", self);
    }

    fn vomit(&mut self, con_id: ConstructId, from: ConstructId) -> String {
        let from_scope = self.constructs[from].scope.dyn_clone();
        let mut paths = PathOverlay::new(self);
        format!("{:?}", paths.get_path(con_id, &*from_scope))
    }
}
