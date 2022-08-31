use typed_arena::Arena;

use super::{Environment, ItemPtr};
use crate::{
    item::{
        definitions::variable::{SVariableInvariants, VariablePtr},
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
    pub fn with_new_self<T>(
        pc: &ParseContext,
        scope: &dyn Scope,
        f: impl for<'xx, 'yy> FnOnce(VomitContext<'xx, 'yy>) -> T,
    ) -> T {
        f(VomitContext {
            pc,
            code_arena: &Arena::new(),
            scope,
            temp_names: &mut OrderedMap::new(),
            anon_name_counter: &mut 0,
        })
    }

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
        _env: &mut Environment,
        of: ItemPtr,
        make_node: impl FnOnce() -> Node<'x>,
    ) -> &'x str {
        let of = of.dereference();
        if let Some(name) = self.temp_names.get(&of) {
            name.0
        } else {
            let name = if let Some(name) = &of.borrow().name {
                let base_name = name.clone();
                let mut name = base_name.clone();
                let mut index = 0;
                while self.temp_names.iter().any(|x| x.1 .0 == name)
                    || self.scope.lookup_ident(&name).map(|x| x.is_some()) == Ok(true)
                {
                    index += 1;
                    name = format!("{}_{}", base_name, index);
                }
                name
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
        let mut to_vomit: Vec<(ItemPtr, ItemPtr)> = Vec::new();
        root.for_self_and_deep_contents(&mut |item| {
            if item.borrow().show && !to_vomit.iter().any(|x| x.0.is_same_instance_as(item)) {
                to_vomit.push((item.ptr_clone(), item.ptr_clone()));
            }
        });
        for (item_id, from) in to_vomit {
            println!();
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
        let original_vomit = self.vomit(255, &mut ctx, item_id.ptr_clone(), false);
        let original_vomit = Self::format_vomit_output(&ctx, original_vomit);
        result.push_str(&format!("{}", original_vomit));

        let inv_from = SWithParent(SVariableInvariants(item_id.ptr_clone()), from_item);
        let mut inv_ctx = VomitContext {
            pc: &pc,
            code_arena: &code_arena,
            scope: &inv_from,
            temp_names: &mut temp_names,
            anon_name_counter: &mut 0,
        };
        result.push_str("\nproves:");
        let set_ptr = item_id.get_invariants().unwrap();
        let set = set_ptr.borrow();
        for invariant in set.statements() {
            let vomited = self.vomit(255, &mut inv_ctx, invariant.ptr_clone(), true);
            let vomited = Self::format_vomit_output(&inv_ctx, vomited);
            result.push_str(&format!("\n    {} ", indented(&vomited,),));
        }
        result.push_str(&format!("\ndepends on: "));
        for dep in item_id.get_dependencies().into_variables() {
            let vomited = self.vomit_var(&mut inv_ctx, dep.var.ptr_clone());
            let vomited = Self::format_vomit_output(&inv_ctx, vomited);
            if !dep.affects_return_value {
                result.push_str("*");
            }
            result.push_str(&format!("{}   ", indented(&vomited)));
        }
        result.push_str(&format!("\nrequires: "));
        for req in item_id.get_dependencies().as_requirements() {
            let vomited = self.vomit(255, &mut inv_ctx, req.statement.ptr_clone(), true);
            let vomited = Self::format_vomit_output(&inv_ctx, vomited);
            result.push_str(&format!(
                "\n    PROOF({}) (specifically, a proof of {})",
                req.statement_text,
                indented(&vomited)
            ));
        }

        result.push_str(&Self::format_vomit_temp_names(&inv_ctx));
        result
    }

    fn format_vomit_output(ctx: &VomitContext, output: Node) -> String {
        output.vomit(&ctx.pc)
    }

    fn format_vomit_temp_names(ctx: &VomitContext) -> String {
        if ctx.temp_names.len() == 0 {
            return format!("");
        }
        let mut result = String::new();
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

    pub fn show_var(&mut self, var: VariablePtr, from: ItemPtr) -> String {
        self.show(var.borrow().item().ptr_clone(), from)
    }

    pub fn vomit_var<'a>(&mut self, ctx: &mut VomitContext<'a, '_>, var: VariablePtr) -> Node<'a> {
        self.vomit(255, ctx, var.borrow().item().ptr_clone(), true)
    }

    pub fn vomit<'a>(
        &mut self,
        max_precedence: u8,
        ctx: &mut VomitContext<'a, '_>,
        item_ptr: ItemPtr,
        allow_identifiers: bool,
    ) -> Node<'a> {
        let mut err = None;
        let item_ptr = item_ptr.dereference();
        if let Some(_) = item_ptr.downcast_definition::<DResolvable>() {
            return Node {
                phrase: "identifier",
                children: vec![NodeChild::Text("UNRESOLVED")],
                ..Default::default()
            };
        }
        if allow_identifiers {
            if let Ok(Some(ident)) = ctx.scope.reverse_lookup_ident(self, item_ptr.dereference()) {
                return Node {
                    phrase: "identifier",
                    children: vec![NodeChild::Text(ctx.code_arena.alloc(ident))],
                    ..Default::default()
                };
            }
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
        if let Some(_err) = err {
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
