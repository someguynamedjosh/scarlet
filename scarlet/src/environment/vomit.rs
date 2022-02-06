use std::ops::ControlFlow;

use typed_arena::Arena;

use super::{ConstructId, Environment};
use crate::{
    constructs::{
        downcast_construct,
        shown::CShown,
        substitution::SubExpr,
        variable::{CVariable, SVariableInvariants, VariableId},
        Construct, ConstructDefinition,
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
        let original_vomit = self.vomit(255, &pc, &code_arena, con_id, &*from).vomit(&pc);
        result.push_str(&format!("{} ({:?})\n", original_vomit, con_id));
        result.push_str(&format!("proves:\n"));
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
            result.push_str("    depending on:\n");
            for dep in invariant.dependencies {
                result.push_str(&format!(
                    "        {} ({:?})\n",
                    indented(&self.vomit(255, &pc, &code_arena, dep, &inv_from).vomit(&pc)),
                    dep,
                ));
            }
        }
        result.push_str(&format!("depends on:\n"));
        for dep in self.get_dependencies(con_id).into_variables() {
            result.push_str(&format!(
                "    {}\n",
                indented(&format!(
                    "{}",
                    self.vomit_var(&pc, &code_arena, dep.id, &inv_from)
                        .vomit(&pc)
                ))
            ));
        }
        result
    }

    pub fn show_var(&mut self, var: VariableId, from: ConstructId) -> String {
        self.for_each_construct(|env, id| {
            if let Some(other_var) = env.get_and_downcast_construct_definition::<CVariable>(id) {
                if var == other_var.get_id() {
                    return ControlFlow::Break(env.show(id, from));
                }
            }
            ControlFlow::Continue(())
        })
        .unwrap_or_else(|| panic!("Variable does not exist!"))
    }

    pub fn vomit_var<'a>(
        &mut self,
        pc: &ParseContext,
        code_arena: &'a Arena<String>,
        var: VariableId,
        from: &dyn Scope,
    ) -> Node<'a> {
        let base = self
            .for_each_construct(|env, id| {
                if let Some(other_var) = env.get_and_downcast_construct_definition::<CVariable>(id)
                {
                    if var == other_var.get_id() {
                        return ControlFlow::Break(id);
                    }
                }
                ControlFlow::Continue(())
            })
            .unwrap_or_else(|| panic!("Variable {:?} does not exist!", var));
        let id = self.push_other(base, from.dyn_clone());
        self.vomit(255, pc, code_arena, id, from)
    }

    pub fn vomit<'a>(
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
}
