use typed_arena::Arena;

use super::{path::PathOverlay, ConstructId, Environment};
use crate::{
    constructs::{
        downcast_construct,
        shown::CShown,
        variable::{CVariable, SVariableInvariants},
        ConstructDefinition,
    },
    parser::{Node, ParseContext},
    scope::{SWithParent, Scope},
    shared::{indented, TripleBool},
};

impl<'x> Environment<'x> {
    pub fn show_all_requested(&mut self) {
        let mut to_vomit = Vec::new();
        for (from, acon) in &self.constructs {
            if let ConstructDefinition::Resolved(con) = &acon.definition {
                if let Some(shown) = downcast_construct::<CShown>(&**con) {
                    let base = shown.get_base();
                    to_vomit.push((base, base));
                }
            }
        }
        for (con_id, from) in to_vomit {
            println!("{}", self.show(con_id, from));
        }
    }

    pub fn show(&mut self, con_id: ConstructId, from_con: ConstructId) -> String {
        let mut result = String::new();

        let from = self.constructs[from_con].scope.dyn_clone();
        let inv_from = SWithParent(SVariableInvariants(con_id), from_con);
        let code_arena = Arena::new();
        let pc = ParseContext::new();
        self.use_reduced_definitions_while_vomiting = false;
        let original_vomit = self.vomit(255, &pc, &code_arena, con_id, &*from).vomit(&pc);
        self.use_reduced_definitions_while_vomiting = true;
        let reduced_vomit = self.vomit(255, &pc, &code_arena, con_id, &*from).vomit(&pc);
        result.push_str(&format!("({:?})\n", con_id));
        result.push_str(&format!("{}\n", original_vomit));
        result.push_str(&format!("reduces to:\n"));
        result.push_str(&format!("{}\n", reduced_vomit));
        result.push_str(&format!("proves:\n"));
        self.use_reduced_definitions_while_vomiting = false;
        for invariant in self.generated_invariants(con_id, &[]) {
            result.push_str(&format!(
                "    {} ({:?} from",
                indented(&format!(
                    "{:?}",
                    self.vomit(255, &pc, &code_arena, invariant.statement, &inv_from)
                        .vomit(&pc)
                )),
                invariant.statement,
            ));
            for &just in &invariant.justified_by {
                result.push_str(&format!(" {:?}", just))
            }
            result.push_str(")\n");
        }
        result.push_str(&format!("depends on:\n"));
        for dep in self.get_dependencies(con_id).into_variables() {
            result.push_str(&format!(
                "    {}\n",
                indented(&format!(
                    "{}",
                    self.vomit_var(&pc, &code_arena, &dep, &inv_from).vomit(&pc)
                ))
            ));
        }
        result
    }

    pub fn show_var(&mut self, var: &CVariable, from: ConstructId) -> String {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            if self.constructs[id]
                .definition
                .as_resolved()
                .map(|con| con.dyn_clone())
                .map(|con| con.is_def_equal(self, var))
                == Some(TripleBool::True)
            {
                return self.show(id, from);
            } else {
                next_id = self.constructs.next(id);
            }
        }
        panic!("Variable does not exist.")
    }

    pub fn vomit_var<'a>(
        &mut self,
        pc: &ParseContext,
        code_arena: &'a Arena<String>,
        var: &CVariable,
        from: &dyn Scope,
    ) -> Node<'a> {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            if self.constructs[id]
                .definition
                .as_resolved()
                .map(|con| con.dyn_clone())
                .map(|con| con.is_def_equal(self, var))
                == Some(TripleBool::True)
            {
                return self.vomit(255, pc, code_arena, id, from);
            } else {
                next_id = self.constructs.next(id);
            }
        }
        panic!("Variable does not exist.")
    }

    pub fn vomit<'a>(
        &mut self,
        max_precedence: u8,
        pc: &ParseContext,
        code_arena: &'a Arena<String>,
        con_id: ConstructId,
        from: &dyn Scope,
    ) -> Node<'a> {
        if let Some(value) = self.get_path(con_id, from, max_precedence, code_arena) {
            return value;
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

    fn get_path<'a>(
        &mut self,
        con_id: ConstructId,
        from: &dyn Scope,
        max_precedence: u8,
        code_arena: &'a Arena<String>,
    ) -> Option<Node<'a>> {
        let mut next_original_id = self.constructs.first();
        let mut shortest_path: Option<Node> = None;
        while let Some(original_id) = next_original_id {
            if self.is_def_equal_for_vomiting(original_id, con_id) == TripleBool::True
                && con_id != original_id
            {
                let mut paths = PathOverlay::new(self);
                let path = paths.get_path(original_id, &*from);
                if let Some(path) = path {
                    if max_precedence >= 4 {
                        let path = path.vomit(code_arena);
                        if shortest_path
                            .as_ref()
                            .map(|p| format!("{:?}", p).len() > format!("{:?}", path).len())
                            .unwrap_or(true)
                        {
                            shortest_path = Some(path)
                        }
                    }
                }
            }
            next_original_id = self.constructs.next(original_id);
        }
        shortest_path
    }
}
