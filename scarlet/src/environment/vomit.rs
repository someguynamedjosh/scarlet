use std::ops::ControlFlow;

use typed_arena::Arena;

use super::{Environment, ItemId, UnresolvedItemError};
use crate::{
    constructs::{
        downcast_construct,
        shown::CShown,
        variable::{CVariable, SVariableInvariants, VariableId},
        Construct, ItemDefinition,
    },
    parser::{Node, ParseContext},
    scope::{SWithParent, Scope},
    shared::indented,
};

impl<'x> Environment<'x> {
    pub fn show_all_requested(&mut self) {
        let mut to_vomit = Vec::new();
        for (from, aitem) in &self.items {
            if let ItemDefinition::Resolved(con) = &aitem.definition {
                if let Some(shown) = downcast_construct::<CShown>(&**con) {
                    let base = shown.get_base();
                    to_vomit.push((base, from));
                }
            }
        }
        for (item_id, from) in to_vomit {
            println!(
                "{}",
                self.show(item_id, from).unwrap_or(format!("Unresolved"))
            );
        }
    }

    pub fn show(
        &mut self,
        item_id: ItemId,
        from_item: ItemId,
    ) -> Result<String, UnresolvedItemError> {
        let mut result = String::new();

        let from = self.items[from_item].scope.dyn_clone();
        let inv_from = SWithParent(SVariableInvariants(item_id), from_item);
        let code_arena = Arena::new();
        let pc = ParseContext::new();
        let original_vomit = self
            .vomit(255, &pc, &code_arena, item_id, &*from)?
            .vomit(&pc);
        result.push_str(&format!("{} ({:?})\n", original_vomit, item_id));
        result.push_str(&format!("proves:\n"));
        for invariant in self.generated_invariants(item_id) {
            result.push_str(&format!(
                "    {} ({:?})\n",
                indented(
                    &self
                        .vomit(255, &pc, &code_arena, invariant.statement, &inv_from)?
                        .vomit(&pc)
                ),
                invariant.statement,
            ));
            result.push_str("    depending on:\n");
            for dep in invariant.dependencies {
                result.push_str(&format!(
                    "        {} ({:?})\n",
                    indented(
                        &self
                            .vomit(255, &pc, &code_arena, dep, &inv_from)?
                            .vomit(&pc)
                    ),
                    dep,
                ));
            }
        }
        result.push_str(&format!("depends on:\n"));
        for dep in self.get_dependencies(item_id).into_variables() {
            result.push_str(&format!(
                "    {}\n",
                indented(&format!(
                    "{}",
                    self.vomit_var(&pc, &code_arena, dep.id, &inv_from)?
                        .vomit(&pc)
                ))
            ));
        }
        Ok(result)
    }

    pub fn show_var(
        &mut self,
        var: VariableId,
        from: ItemId,
    ) -> Result<String, UnresolvedItemError> {
        self.for_each_item(|env, id| {
            if let Ok(Some(other_var)) = env.get_and_downcast_construct_definition::<CVariable>(id)
            {
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
    ) -> Result<Node<'a>, UnresolvedItemError> {
        let base = self
            .for_each_item(|env, id| {
                if let Ok(Some(other_var)) =
                    env.get_and_downcast_construct_definition::<CVariable>(id)
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
        item_id: ItemId,
        from: &dyn Scope,
    ) -> Result<Node<'a>, UnresolvedItemError> {
        for (_, phrase) in &pc.phrases_sorted_by_vomit_priority {
            if phrase.precedence > max_precedence {
                continue;
            }
            if let Some((_, uncreator)) = phrase.create_and_uncreate {
                if let Some(uncreated) = uncreator(pc, self, code_arena, item_id, from)? {
                    return Ok(uncreated);
                }
            }
        }
        eprintln!("{:#?}", self);
        todo!(
            "{:?} could not be vomited (at least one parser phrase should apply)",
            item_id
        );
    }
}
