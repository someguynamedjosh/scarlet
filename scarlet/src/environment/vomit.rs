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
    shared::{indented, OrderedMap},
};

pub struct VomitContext<'x, 'y> {
    pub pc: &'x ParseContext,
    pub code_arena: &'x Arena<String>,
    pub scope: &'y dyn Scope,
    pub temp_names: &'y mut OrderedMap<ItemId, (&'x str, Node<'x>)>,
    pub anon_name_counter: usize,
}

impl<'x, 'y> VomitContext<'x, 'y> {
    pub fn get_name(
        &mut self,
        env: &mut Environment,
        of: ItemId,
        make_node: impl FnOnce() -> Node<'x>,
    ) -> &'x str {
        if let Some(name) = self.temp_names.get(&of) {
            name.0
        } else {
            let anon_name = self
                .code_arena
                .alloc(format!("anon_{}", self.anon_name_counter));
            self.temp_names
                .insert_no_replace(of, (&*anon_name, make_node()));
            self.anon_name_counter += 1;
            &*anon_name
        }
    }
}

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
        let code_arena = Arena::new();
        let pc = ParseContext::new();
        let mut temp_names = OrderedMap::new();
        let mut ctx = VomitContext {
            pc: &pc,
            code_arena: &code_arena,
            scope: &*from,
            temp_names: &mut temp_names,
            anon_name_counter: 0,
        };
        let original_vomit = self.vomit(255, &mut ctx, item_id)?;
        let original_vomit = Self::format_vomit_output(&ctx, original_vomit);
        result.push_str(&format!("{} ({:?})\n", original_vomit, item_id));
        result.push_str(&format!("proves:\n"));

        let inv_from = SWithParent(SVariableInvariants(item_id), from_item);
        let mut inv_ctx = VomitContext {
            pc: &pc,
            code_arena: &code_arena,
            scope: &inv_from,
            temp_names: &mut temp_names,
            anon_name_counter: 0,
        };
        for invariant in self.generated_invariants(item_id) {
            let vomited = self.vomit(255, &mut inv_ctx, invariant.statement)?;
            let vomited = Self::format_vomit_output(&inv_ctx, vomited);
            result.push_str(&format!(
                "    {} ({:?})\n",
                indented(&vomited,),
                invariant.statement,
            ));
            result.push_str("    depending on:\n");
            for dep in invariant.dependencies {
                let vomited = self.vomit(255, &mut inv_ctx, dep)?;
                let vomited = Self::format_vomit_output(&inv_ctx, vomited);
                result.push_str(&format!("        {} ({:?})\n", indented(&vomited), dep,));
            }
        }
        result.push_str(&format!("depends on:\n"));
        for dep in self.get_dependencies(item_id).into_variables() {
            let vomited = self.vomit_var(&mut inv_ctx, dep.id)?;
            let vomited = Self::format_vomit_output(&inv_ctx, vomited);
            result.push_str(&format!("    {}\n", indented(&vomited)));
        }
        Ok(result)
    }

    fn format_vomit_output(ctx: &VomitContext, output: Node) -> String {
        let base = output.vomit(&ctx.pc);
        if ctx.temp_names.len() == 0 {
            base
        } else {
            let mut result = base;
            result.push_str("\nUSING {");
            for (_id, (name, node)) in &*ctx.temp_names {
                result.push_str("\n    ");
                result.push_str(name);
                result.push_str(" IS ");
                result.push_str(&node.vomit(&ctx.pc));
            }
            result.push_str("\n}");
            result
        }
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
        ctx: &mut VomitContext<'a, '_>,
        var: VariableId,
    ) -> Result<Node<'a>, UnresolvedItemError> {
        let id = self.variables[var].item.unwrap();
        self.vomit(255, ctx, id)
    }

    pub fn vomit<'a>(
        &mut self,
        max_precedence: u8,
        ctx: &mut VomitContext<'a, '_>,
        item_id: ItemId,
    ) -> Result<Node<'a>, UnresolvedItemError> {
        for (_, phrase) in &ctx.pc.phrases_sorted_by_vomit_priority {
            if phrase.precedence > max_precedence {
                continue;
            }
            if let Some((_, uncreator)) = phrase.create_and_uncreate {
                if let Some(uncreated) = uncreator(self, ctx, item_id)? {
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
