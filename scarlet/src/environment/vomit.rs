use std::ops::ControlFlow;

use typed_arena::Arena;

use super::{Environment, ItemPtr};
use crate::{
    item::{
        definition::ItemDefinition,
        definitions::variable::{DVariable, SVariableInvariants, VariablePtr},
        resolvable::DResolvable,
    },
    parser::{Node, NodeChild, ParseContext},
    scope::{SWithParent, Scope},
    shared::{indented, OrderedMap},
    util::PtrExtension,
};

pub struct VomitContext<'x, 'y> {
    pub pc: &'x ParseContext,
    pub code_arena: &'x Arena<String>,
    pub scope: &'y dyn Scope,
    pub temp_names: &'y mut OrderedMap<ItemPtr, (&'x str, Node<'x>)>,
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
        of: ItemPtr,
        make_node: impl FnOnce() -> Node<'x>,
    ) -> &'x str {
        let of = of.dereference();
        if let Some(name) = self.temp_names.get(&of) {
            name.0
        } else {
            let name = if let Some(name) = &of.borrow().name {
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

impl Environment {
    pub fn show_all_requested(&mut self, root: &ItemPtr) {
        let mut to_vomit = Vec::new();
        root.for_self_and_contents(&mut |item| {
            if item.borrow().show {
                to_vomit.push((item.ptr_clone(), item.ptr_clone()));
            }
        });
        for (item_id, from) in to_vomit {
            println!("{}", self.show(item_id, from));
        }
    }

    pub fn show(&mut self, item_id: ItemPtr, from_item: ItemPtr) -> String {
        let mut result = String::new();

        let from = from_item.clone_scope();
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
        let original_vomit = self.vomit(255, &mut ctx, item_id.ptr_clone());
        let original_vomit = Self::format_vomit_output(&ctx, original_vomit);
        result.push_str(&format!("{}\n", original_vomit));
        result.push_str(&format!("proves:"));

        let inv_from = SWithParent(SVariableInvariants(item_id.ptr_clone()), from_item);
        temp_names.clear();
        let mut inv_ctx = VomitContext {
            pc: &pc,
            code_arena: &code_arena,
            scope: &inv_from,
            temp_names: &mut temp_names,
            anon_name_counter: &mut 0,
        };
        let set_ptr = item_id.get_invariants().unwrap();
        let set = set_ptr.borrow();
        for invariant in set.statements() {
            let vomited = self.vomit(255, &mut inv_ctx, invariant.ptr_clone());
            inv_ctx.temp_names.clear();
            let vomited = Self::format_vomit_output(&inv_ctx, vomited);
            result.push_str(&format!("\n    {} ", indented(&vomited,),));
        }
        for dep in set.dependencies() {
            let vomited = self.vomit(255, &mut inv_ctx, dep.ptr_clone());
            inv_ctx.temp_names.clear();
            let vomited = Self::format_vomit_output(&inv_ctx, vomited);
            result.push_str(&format!("{}   ", indented(&vomited)));
        }
        result.push_str(&format!("\ndepends on: "));
        for dep in item_id.get_dependencies().into_variables() {
            let vomited = self.vomit_var(&mut inv_ctx, dep.var.ptr_clone());
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

    pub fn show_var(&mut self, var: VariablePtr, from: ItemPtr) -> String {
        self.show(var.borrow().item().ptr_clone(), from)
    }

    pub fn vomit_var<'a>(&mut self, ctx: &mut VomitContext<'a, '_>, var: VariablePtr) -> Node<'a> {
        self.vomit(255, ctx, var.borrow().item().ptr_clone())
    }

    pub fn vomit<'a>(
        &mut self,
        max_precedence: u8,
        ctx: &mut VomitContext<'a, '_>,
        item_ptr: ItemPtr,
    ) -> Node<'a> {
        let mut err = None;
        if let Some(_) = item_ptr.downcast_definition::<DResolvable>() {
            return Node {
                phrase: "identifier",
                children: vec![NodeChild::Text("UNRESOLVED")],
                ..Default::default()
            };
        }
        for (_, phrase) in &ctx.pc.phrases_sorted_by_vomit_priority {
            if phrase.precedence > max_precedence {
                continue;
            }
            if let Some((_, uncreator)) = phrase.create_and_uncreate {
                match uncreator(self, ctx, item_ptr.ptr_clone()) {
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
            item_ptr
        );
    }
}
