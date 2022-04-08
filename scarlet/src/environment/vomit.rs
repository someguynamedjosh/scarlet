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
    parser::{Node, NodeChild, ParseContext},
    scope::{SWithParent, Scope},
    shared::{indented, OrderedMap},
};

pub struct VomitContext<'x, 'y> {
    pub pc: &'x ParseContext,
    pub code_arena: &'x Arena<String>,
    pub scope: &'y dyn Scope,
    pub temp_names: &'y mut OrderedMap<ItemId, (&'x str, Node<'x>)>,
    pub anon_name_counter: &'y mut usize,
}

impl<'x, 'y> VomitContext<'x, 'y> {
    pub fn with_scope<'yy>(&'yy mut self, scope: &'yy dyn Scope) -> VomitContext<'x, 'yy>
    where
        'y: 'yy,
    {
        VomitContext {
            scope,
            temp_names: &mut *self.temp_names,
            anon_name_counter: &mut *self.anon_name_counter,
            ..*self
        }
    }

    pub fn get_name(
        &mut self,
        env: &mut Environment,
        of: ItemId,
        make_node: impl FnOnce() -> Node<'x>,
    ) -> &'x str {
        let of = env.dereference(of).unwrap_or(of);
        if let Some(name) = self.temp_names.get(&of) {
            name.0
        } else {
            let name = if let Some(name) = &env.items[of].name {
                name.clone()
            } else {
                format!("anon_{}", self.anon_name_counter)
            };
            let name = self.code_arena.alloc(name);
            self.temp_names.insert_no_replace(of, (&*name, make_node()));
            *self.anon_name_counter += 1;
            &*name
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
            println!("{}", self.show(item_id, from));
        }
    }

    pub fn show(&mut self, item_id: ItemId, from_item: ItemId) -> String {
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
            anon_name_counter: &mut 0,
        };
        let original_vomit = self.vomit(255, &mut ctx, item_id);
        let original_vomit = Self::format_vomit_output(&ctx, original_vomit);
        result.push_str(&format!("{}\n", original_vomit));
        result.push_str(&format!("proves:"));

        let inv_from = SWithParent(SVariableInvariants(item_id), from_item);
        temp_names.clear();
        let mut inv_ctx = VomitContext {
            pc: &pc,
            code_arena: &code_arena,
            scope: &inv_from,
            temp_names: &mut temp_names,
            anon_name_counter: &mut 0,
        };
        // for invariant in self.generated_invariants(item_id) {
        //     let vomited = self.vomit(255, &mut inv_ctx, invariant.statement);
        //     inv_ctx.temp_names.clear();
        //     let vomited = Self::format_vomit_output(&inv_ctx, vomited);
        //     result.push_str(&format!("\n    {} dep: ", indented(&vomited,),));
        //     for dep in invariant.dependencies {
        //         let vomited = self.vomit(255, &mut inv_ctx, dep);
        //         inv_ctx.temp_names.clear();
        //         let vomited = Self::format_vomit_output(&inv_ctx, vomited);
        //         result.push_str(&format!("{}   ", indented(&vomited)));
        //     }
        // }
        result.push_str(&format!("\ndepends on: "));
        for dep in self.get_dependencies(item_id).into_variables() {
            let vomited = self.vomit_var(&mut inv_ctx, dep.id);
            inv_ctx.temp_names.clear();
            let vomited = Self::format_vomit_output(&inv_ctx, vomited);
            result.push_str(&format!("{}   ", indented(&vomited)));
        }
        result
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

    pub fn show_var(&mut self, var: VariableId, from: ItemId) -> String {
        let id = self.variables[var].item.unwrap();
        self.show(id, from)
    }

    pub fn vomit_var<'a>(&mut self, ctx: &mut VomitContext<'a, '_>, var: VariableId) -> Node<'a> {
        let id = self.variables[var].item.unwrap();
        self.vomit(255, ctx, id)
    }

    pub fn vomit<'a>(
        &mut self,
        max_precedence: u8,
        ctx: &mut VomitContext<'a, '_>,
        item_id: ItemId,
    ) -> Node<'a> {
        let mut err = None;
        for (_, phrase) in &ctx.pc.phrases_sorted_by_vomit_priority {
            if phrase.precedence > max_precedence {
                continue;
            }
            if let Some((_, uncreator)) = phrase.create_and_uncreate {
                match uncreator(self, ctx, item_id) {
                    Err(new_err) => err = Some(new_err),
                    Ok(Some(uncreated)) => return uncreated,
                    _ => (),
                }
            }
        }
        if let Some(err) = err {
            return Node {
                phrase: "identifier",
                children: vec![NodeChild::Text("UNRESOLVED")],
                ..Default::default()
            };
        }
        eprintln!("{:#?}", self);
        todo!(
            "{:?} could not be vomited (at least one parser phrase should apply)",
            item_id
        );
    }
}
