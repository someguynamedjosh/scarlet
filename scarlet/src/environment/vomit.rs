use typed_arena::Arena;

use super::{path::PathOverlay, Construct, ConstructId, Environment};
use crate::{
    constructs::{downcast_construct, shown::CShown, variable::CVariable, ConstructDefinition},
    parser::{Node, ParseContext},
    shared::{indented, TripleBool},
};

impl<'x> Environment<'x> {
    pub fn show_all_requested(&mut self) {
        let code_arena = Arena::new();
        let mut to_vomit = Vec::new();
        for (from, acon) in &self.constructs {
            if let ConstructDefinition::Resolved(con) = &acon.definition {
                if let Some(shown) = downcast_construct::<CShown>(&**con) {
                    let base = shown.get_base();
                    to_vomit.push((base, from));
                }
            }
        }
        let pc = ParseContext::new();
        for (con_id, from) in to_vomit {
            let vomited = self.vomit(255, &pc, &code_arena, con_id, from);
            println!("{:?} is\n{:?}", con_id, vomited);
            println!("proves:");
            for invariant in self.generated_invariants(con_id) {
                println!(
                    "    {:?}",
                    self.vomit(255, &pc, &code_arena, invariant, from)
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
                    indented(&format!(
                        "{:?}",
                        self.vomit_var(&pc, &code_arena, dep, from)
                    ))
                );
            }
            println!();
        }
    }

    fn vomit_var<'a>(
        &mut self,
        pc: &ParseContext,
        code_arena: &'a Arena<String>,
        var: CVariable,
        from: ConstructId,
    ) -> Node<'a> {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            if self.constructs[id]
                .definition
                .as_resolved()
                .map(|con| con.dyn_clone())
                .map(|con| con.is_def_equal(self, &var))
                == Some(TripleBool::True)
            {
                return self.vomit(255, pc, code_arena, id, from);
            } else {
                next_id = self.constructs.next(id);
            }
        }
        panic!("Variable does not exist.")
    }

    fn vomit<'a>(
        &mut self,
        max_precedence: u8,
        pc: &ParseContext,
        code_arena: &'a Arena<String>,
        con_id: ConstructId,
        from: ConstructId,
    ) -> Node<'a> {
        let con_id = self.dereference(con_id);
        let from_scope = self.constructs[from].scope.dyn_clone();
        let mut next_original_id = self.constructs.first();
        while let Some(original_id) = next_original_id {
            if self.is_def_equal(con_id, original_id) == TripleBool::True {
                let mut paths = PathOverlay::new(self);
                let path = paths.get_path(original_id, &*from_scope);
                if let Some(path) = path {
                    if max_precedence >= 4 {
                        return path.vomit(code_arena);
                    }
                }
            }
            next_original_id = self.constructs.next(original_id);
        }
        for (_, phrase) in &pc.phrases {
            if phrase.precedence > max_precedence {
                continue;
            }
            if let Some((_, uncreator)) = phrase.create_and_uncreate {
                if let Some(uncreated) = uncreator(pc, self, code_arena, con_id, from) {
                    return uncreated;
                }
            }
        }
        println!("{:#?}", self);
        todo!(
            "{:?} could not be vomited (at least one parser phrase should apply)",
            con_id
        );
    }
}
