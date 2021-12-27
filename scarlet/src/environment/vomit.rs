use typed_arena::Arena;

use super::{path::PathOverlay, Construct, ConstructId, Environment};
use crate::{
    constructs::{downcast_construct, shown::CShown, variable::CVariable, ConstructDefinition},
    shared::{indented, TripleBool},
};

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
        for (con_id, from) in to_vomit {
            let vomited = self.vomit(con_id, from);
            println!(
                "{:?} is\n{}",
                con_id,
                vomited.unwrap_or("anonymous".to_owned())
            );
            println!("proves:");
            for invariant in self.generated_invariants(con_id) {
                println!(
                    "    {}",
                    self.vomit(invariant, from)
                        .unwrap_or("anonymous".to_owned())
                );
            }
            println!("depends on:");
            for dep in self.get_dependencies(con_id) {
                let kind = match dep.is_capturing() {
                    true => "capturing",
                    false => "without capturing",
                };
                println!(
                    "    {} {}",
                    kind,
                    indented(&self.vomit_var(dep, from).unwrap_or("anonymous".to_owned()))
                );
            }
            println!();
        }
    }

    fn vomit_var(&mut self, var: CVariable, from: ConstructId) -> Option<String> {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            if self.constructs[id]
                .definition
                .as_resolved()
                .map(|con| con.dyn_clone())
                .map(|con| con.is_def_equal(self, &var))
                == Some(TripleBool::True)
            {
                return self.vomit(id, from);
            } else {
                next_id = self.constructs.next(id);
            }
        }
        None
    }

    fn vomit(&mut self, con_id: ConstructId, from: ConstructId) -> Option<String> {
        let code_arena = Arena::new();
        let con_id = self.dereference(con_id);
        let from_scope = self.constructs[from].scope.dyn_clone();
        let mut next_original_id = self.constructs.first();
        while let Some(original_id) = next_original_id {
            if self.is_def_equal(con_id, original_id) == TripleBool::True {
                let mut paths = PathOverlay::new(self);
                let path = paths.get_path(original_id, &*from_scope);
                if let Some(path) = path {
                    let path = path.vomit(&code_arena);
                    return Some(format!("{:?}", path));
                }
            }
            next_original_id = self.constructs.next(original_id);
        }
        None
    }
}
