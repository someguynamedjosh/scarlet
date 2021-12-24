use typed_arena::Arena;

use super::{path::PathOverlay, Construct, ConstructId, Environment};
use crate::constructs::{downcast_construct, shown::CShown, ConstructDefinition};

impl<'x> Environment<'x> {
    pub fn show_all_requested(&mut self) {
        let mut to_vomit = Vec::new();
        for (from, acon) in &self.constructs {
            if let ConstructDefinition::Resolved(con) = &acon.definition {
                if let Some(shown) = downcast_construct::<CShown>(&**con) {
                    let base = shown.get_base();
                    to_vomit.push((base, from));
                }
            }
        }
        let code_arena = Arena::new();
        for (con_id, from) in to_vomit {
            let vomited = self.vomit(&code_arena, con_id, from);
            println!(
                "{:?} is\n{}",
                con_id,
                vomited.unwrap_or("anonymous".to_owned())
            );
            println!("depends on:");
            for dep in self.get_dependencies(con_id) {
                let kind = match dep.is_capturing() {
                    true => "capturing",
                    false => "without capturing",
                };
                println!("    {} (", kind);
                for inv in dep.get_invariants() {
                    println!(
                        "        {}",
                        self.vomit(&code_arena, *inv, from)
                            .unwrap_or("anonymous".to_owned())
                    );
                }
                println!("    )");
            }
            println!();
        }
        // println!("{:#?}", self);
    }

    fn vomit(
        &mut self,
        code_arena: &Arena<String>,
        con_id: ConstructId,
        from: ConstructId,
    ) -> Option<String> {
        let con_id = self.dereference(con_id);
        let from_scope = self.constructs[from].scope.dyn_clone();
        let mut paths = PathOverlay::new(self);
        let path = paths.get_path(con_id, &*from_scope)?;
        let path = path.vomit(code_arena);
        Some(format!("{:?}", path))
    }
}
