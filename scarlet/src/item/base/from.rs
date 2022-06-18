use crate::{
    diagnostic::Position,
    environment::Environment,
    item::{
        definitions::{
            builtin_function::DBuiltinFunction, structt::DPopulatedStruct,
            substitution::DSubstitution, variable::DVariable,
        },
        item::ItemPtr,
        resolvable::{from::RFrom, DResolvable, RSubstitution},
        Item, ItemDefinition,
    },
    scope::{SPlain, Scope},
    util::PtrExtension,
};

/// Makes a dex that returns true if the dependency "x" is substituted with a
/// value that could have been returned by `from_item`.
pub(super) fn create_from_dex(env: &Environment, from: ItemPtr, position: Position) -> ItemPtr {
    let scope = || SPlain(from.ptr_clone());
    let into = Item::placeholder_with_scope(Box::new(scope()));
    from.borrow_mut().from_dex = Some(into.ptr_clone());
    let x = env.get_language_item("x").unwrap();

    if let Some(structt) = from.downcast_definition::<DPopulatedStruct>() {
        let structt = structt.clone();

        let has_tail = DBuiltinFunction::has_tail(env, x.ptr_clone(), Box::new(scope()), position);
        let has_tail = Item::new(DResolvable::new(has_tail), scope());

        let x_value = DBuiltinFunction::tail_value(env, x.ptr_clone(), Box::new(scope()), position);
        let x_value = Item::new(DResolvable::new(x_value), scope());
        let value_from_value = RFrom {
            left: x_value,
            right: structt.get_value().ptr_clone(),
        };
        let value_from_value = Item::new(DResolvable::new(value_from_value), scope());

        let x_body = DBuiltinFunction::body(env, x.ptr_clone(), Box::new(scope()), position);
        let x_body = Item::new(DResolvable::new(x_body), scope());
        let rest_from_rest = RFrom {
            left: x_body,
            right: structt.get_rest().ptr_clone(),
        };
        let rest_from_rest = Item::new(DResolvable::new(rest_from_rest), scope());

        let first_two = create_and(env, has_tail, value_from_value, scope());
        redefine_as_and(env, into.ptr_clone(), first_two, rest_from_rest);
    } else if let Some(var) = from.downcast_definition::<DVariable>() {
        let var_ptr = var.get_variable();
        let var = var_ptr.borrow();
        let invs = Vec::from(var.get_invariants());
        let deps = Vec::from(var.get_dependencies());
        if deps.len() > 0 {
            todo!();
        }
        let truee = env.get_true();

        let statement = if invs.len() == 0 {
            truee.ptr_clone()
        } else {
            let mut statement = invs[0].ptr_clone();
            for part in &invs[1..] {
                statement = create_and(env, statement, part.ptr_clone(), scope());
            }
            statement
        };

        let subs = vec![(var_ptr.ptr_clone(), x.ptr_clone())]
            .into_iter()
            .collect();
        let con = DSubstitution::new_unchecked(statement, subs);
        into.redefine(con.clone_into_box());
    } else {
        let truee = env.get_true();
        let falsee = env.get_false();
        let equal = DBuiltinFunction::decision(
            env,
            x.ptr_clone(),
            from.ptr_clone(),
            truee.ptr_clone(),
            falsee.ptr_clone(),
            Box::new(scope()),
            position,
        );
        into.redefine(Box::new(DResolvable::new(equal)));
    }
    into
}

fn create_and(
    env: &Environment,
    left: ItemPtr,
    right: ItemPtr,
    scope: impl Scope + 'static,
) -> ItemPtr {
    let and = env.get_language_item("and").unwrap();
    Item::new(
        DResolvable::new(RSubstitution {
            base: and.ptr_clone(),
            position: Position::placeholder(),
            named_subs: vec![].into_iter().collect(),
            anonymous_subs: vec![left, right],
        }),
        scope,
    )
}

fn redefine_as_and(env: &Environment, original: ItemPtr, left: ItemPtr, right: ItemPtr) {
    let and = env.get_language_item("and").unwrap();
    original.redefine(
        DResolvable::new(RSubstitution {
            base: and.ptr_clone(),
            position: Position::placeholder(),
            named_subs: vec![].into_iter().collect(),
            anonymous_subs: vec![left, right],
        })
        .clone_into_box(),
    )
}
