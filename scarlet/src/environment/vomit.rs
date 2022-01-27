use std::ops::ControlFlow;

use typed_arena::Arena;

use super::{ConstructId, Environment};
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
                    to_vomit.push((base, from));
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
        let original_vomit = self
            .vomit_restricting_path(255, &pc, &code_arena, con_id, &*from)
            .vomit(&pc);
        self.use_reduced_definitions_while_vomiting = true;
        let reduced_vomit = self
            .vomit_restricting_path(255, &pc, &code_arena, con_id, &*from)
            .vomit(&pc);
        result.push_str(&format!("{} ({:?})\n", original_vomit, con_id));
        result.push_str(&format!("reduces to:\n"));
        result.push_str(&format!("{}\n", reduced_vomit));
        result.push_str(&format!("proves:\n"));
        self.use_reduced_definitions_while_vomiting = false;
        for invariant in self.generated_invariants(con_id) {
            result.push_str(&format!(
                "    {} ({:?})\n",
                indented(
                    &self
                        .vomit(255, &pc, &code_arena, invariant.statement, &inv_from)
                        .vomit(&pc)
                ),
                invariant.statement,
            ));
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
        self.for_each_construct(|env, id| {
            if env.constructs[id]
                .definition
                .as_resolved()
                .map(|con| con.dyn_clone())
                .map(|con| con.is_def_equal(env, var))
                == Some(TripleBool::True)
            {
                ControlFlow::Break(env.show(id, from))
            } else {
                ControlFlow::Continue(())
            }
        })
        .unwrap_or_else(|| panic!("Variable does not exist!"))
    }

    pub fn vomit_var<'a>(
        &mut self,
        pc: &ParseContext,
        code_arena: &'a Arena<String>,
        var: &CVariable,
        from: &dyn Scope,
    ) -> Node<'a> {
        self.for_each_construct(|env, id| {
            if env.constructs[id]
                .definition
                .as_other()
                .map(|con| env.constructs[con].definition.as_resolved())
                .flatten()
                .map(|con| con.dyn_clone())
                .map(|con| con.is_def_equal(env, var))
                == Some(TripleBool::True)
            {
                ControlFlow::Break(env.vomit(255, pc, code_arena, id, from))
            } else {
                ControlFlow::Continue(())
            }
        })
        .unwrap_or_else(|| panic!("Variable does not exist!"))
    }

    fn vomit_restricting_path<'a>(
        &mut self,
        max_precedence: u8,
        pc: &ParseContext,
        code_arena: &'a Arena<String>,
        con_id: ConstructId,
        from: &dyn Scope,
    ) -> Node<'a> {
        for (_, phrase) in &pc.phrases_sorted_by_vomit_priority {
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

    pub fn vomit<'a>(
        &mut self,
        max_precedence: u8,
        pc: &ParseContext,
        code_arena: &'a Arena<String>,
        con_id: ConstructId,
        from: &dyn Scope,
    ) -> Node<'a> {
        self.vomit_restricting_path(max_precedence, pc, code_arena, con_id, from)
    }
}
