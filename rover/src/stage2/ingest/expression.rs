use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::{
        ingest::{
            helpers::{self, Context},
            statements::{process_definitions, process_replacements},
        },
        structure::{Definitions, Environment, Item, ItemId, PrimitiveValue},
    },
};

fn process_from(
    base_id: ItemId,
    statements: Vec<Statement>,
    env: &mut Environment,
    parents: &[&Definitions],
) -> Result<Item, String> {
    let mut vars = Vec::new();
    let mut named_vars = Vec::new();
    for statement in statements {
        match statement {
            Statement::Expression(expr) => {
                let ctx = Context::Plain;
                let var = process_expr(expr, None, env, ctx, parents)?;
                vars.push(var);
            }
            Statement::Is(is) => {
                let name = is.name.expect_ident_owned()?;
                let expr = is.value;
                let ctx = Context::Plain;
                let var = process_expr(expr, None, env, ctx, parents)?;
                named_vars.push((name, var));
                vars.push(var);
            }
            Statement::Else(..)
            | Statement::PickElif(..)
            | Statement::PickIf(..)
            | Statement::Replace(..) => todo!("nice error"),
        }
    }
    let base_item = Item::FromType {
        base: base_id,
        vars,
    };
    Ok(if named_vars.len() == 0 {
        base_item
    } else {
        let base = env.next_id();
        env.define(base, base_item);
        let definitions = named_vars;
        Item::Defining { base, definitions }
    })
}

fn process_variant(
    variant_name: String,
    typee_id: ItemId,
    env: &mut Environment,
    ctx: Context,
) -> Result<Item, String> {
    let (base_id, where_body, vars) = match env.definition_of(typee_id) {
        None => (typee_id, None, vec![]),
        Some(Item::Defining { base, definitions }) => {
            let vars = match env.definition_of(*base).as_ref().unwrap() {
                Item::FromType { vars, .. } => vars.clone(),
                _ => unreachable!(),
            };
            (*base, Some(definitions), vars)
        }
        Some(Item::FromType { vars, base }) => (*base, None, vars.clone()),
        _ => unreachable!(),
    };
    if let Context::Type(typee) = ctx {
        if base_id != typee {
            todo!("nice error, variant type is not Self.")
        }
        let base = Item::InductiveValue {
            typee,
            variant_name,
            records: vars,
        };
        Ok(if let Some(definitions) = where_body {
            let definitions = definitions.clone();
            let base_into = env.next_id();
            env.define(base_into, base);
            Item::Defining {
                base: base_into,
                definitions,
            }
        } else {
            base
        })
    } else {
        todo!("nice error, variant used outside of Type construct")
    }
}

fn process_postfix(
    post: Construct,
    remainder: Expression,
    env: &mut Environment,
    parents: &[&Definitions],
) -> Result<Item, String> {
    let mut new_parents = parents.to_owned();
    // This makes me uncomfortable.
    let mut cheeky_defining_storage = None;
    match &post.label[..] {
        "defining" => {
            let statements = post.expect_statements("defining")?.to_owned();
            let ctx = Context::Plain;
            let body = process_definitions(statements, vec![], env, ctx, parents)?;
            cheeky_defining_storage = Some(body);
            new_parents.push(cheeky_defining_storage.as_ref().unwrap());
        }
        "replacing" | "member" | "From" | "is_variant" | "type_is" | "type_is_exactly" => (),
        _ => todo!("nice error, unexpected {} construct", post.label),
    }
    let parents = &new_parents[..];
    let base_id = process_expr(remainder, None, env, Context::Plain, parents)?;
    Ok(match &post.label[..] {
        "defining" => {
            let definitions = cheeky_defining_storage.unwrap();
            Item::Defining {
                base: base_id,
                definitions,
            }
        }
        "replacing" => {
            let statements = post.expect_statements("replacing")?.to_owned();
            let (replacements, unlabeled_replacements) =
                process_replacements(statements, env, parents)?;
            Item::Replacing {
                base: base_id,
                replacements,
                unlabeled_replacements,
            }
        }
        "member" => {
            let name = post
                .expect_single_expression("member")?
                .clone()
                .expect_ident_owned()?;
            Item::Member {
                base: base_id,
                name,
            }
        }
        "From" => {
            let statements = post.expect_statements("From")?;
            process_from(base_id, statements.to_owned(), env, parents)?
        }
        "is_variant" => {
            let other = post.expect_single_expression("is_variant")?;
            let ctx = Context::Plain;
            let other = process_expr(other.clone(), None, env, ctx, parents)?;
            Item::IsSameVariant {
                base: base_id,
                other,
            }
        }
        "type_is" => {
            let typee = post.expect_single_expression("type_is")?;
            let ctx = Context::Plain;
            let typee = process_expr(typee.clone(), None, env, ctx, parents)?;
            Item::TypeIs {
                exact: false,
                base: base_id,
                typee,
            }
        }
        "type_is_exactly" => {
            let typee = post.expect_single_expression("type_is_exactly")?;
            let ctx = Context::Plain;
            let typee = process_expr(typee.clone(), None, env, ctx, parents)?;
            Item::TypeIs {
                exact: true,
                base: base_id,
                typee,
            }
        }
        _ => unreachable!(),
    })
}

fn process_type(
    statements: Vec<Statement>,
    into: &mut Option<ItemId>,
    env: &mut Environment,
    parents: &[&Definitions],
) -> Result<Item, String> {
    let into = helpers::get_or_put_into(into, env);
    let type_item = env.next_id();
    let ctx = Context::Type(into);
    let self_def = (format!("Self"), into);
    let definitions = process_definitions(statements, vec![self_def], env, ctx, parents)?;
    env.define(type_item, Item::InductiveType(into));
    Ok(Item::Defining {
        base: type_item,
        definitions,
    })
}

fn process_pick(
    statements: &[Statement],
    env: &mut Environment,
    parents: &[&Definitions],
) -> Result<Item, String> {
    if statements.len() < 2 {
        todo!("nice error, pick must have at least 2 clauses.");
    }

    let initial_clause = if let Statement::PickIf(s) = &statements[0] {
        (
            process_expr(s.condition.clone(), None, env, Context::Plain, parents)?,
            process_expr(s.value.clone(), None, env, Context::Plain, parents)?,
        )
    } else {
        todo!("nice error, first clause must be an if.");
    };

    let last = statements.len() - 1;
    let else_clause = if let Statement::Else(s) = &statements[last] {
        process_expr(s.value.clone(), None, env, Context::Plain, parents)?
    } else {
        todo!("nice error, first clause must be an if.");
    };

    let mut elif_clauses = Vec::new();
    for other in &statements[1..last] {
        if let Statement::PickElif(s) = other {
            elif_clauses.push((
                process_expr(s.condition.clone(), None, env, Context::Plain, parents)?,
                process_expr(s.value.clone(), None, env, Context::Plain, parents)?,
            ));
        } else {
            todo!("nice error, other clauses must be elif");
        }
    }

    Ok(Item::Pick {
        initial_clause,
        elif_clauses,
        else_clause,
    })
}

fn resolve_ident(ident: &str, parents: &[&Definitions]) -> Result<ItemId, String> {
    // Reverse to earch the closest parents first.
    for parent in parents.iter().rev() {
        for (name, val) in *parent {
            if name == ident {
                return Ok(*val);
            }
        }
    }
    Err(format!(
        "Could not find an item named {} in the current scope or its parents.",
        ident
    ))
}

fn process_root(
    root: Construct,
    into: &mut Option<ItemId>,
    env: &mut Environment,
    ctx: Context,
    parents: &[&Definitions],
) -> Result<Item, String> {
    Ok(match &root.label[..] {
        "identifier" => {
            let ident = root.expect_ident()?;
            Item::Item(resolve_ident(ident, parents)?)
        }
        "Type" => {
            let statements = root.expect_statements("Type")?.to_owned();
            process_type(statements, into, env, parents)?
        }
        "any" => {
            let typ_expr = root.expect_single_expression("any")?.clone();
            let typee = process_expr(typ_expr, None, env, Context::Plain, parents)?;
            let selff = helpers::get_or_put_into(into, env);
            Item::Variable { selff, typee }
        }
        "the" => todo!(),
        "i32" => {
            let val = root.expect_text("i32")?;
            let val: i32 = val.parse().unwrap();
            Item::PrimitiveValue(PrimitiveValue::I32(val))
        }
        "variant" => {
            let def_expr = root.expect_single_expression("variant")?;
            let variant_name = def_expr.root.expect_ident()?.to_owned();
            if def_expr.others.len() != 1 {
                todo!("nice error");
            }
            let type_expr = def_expr.others[0]
                .expect_single_expression("type_is")?
                .clone();
            let typee = process_expr(type_expr, None, env, Context::Plain, parents)?;
            process_variant(variant_name, typee, env, ctx)?
        }
        "pick" => {
            let statements = root.expect_statements("pick")?;
            process_pick(statements, env, parents)?
        }
        _ => todo!("nice error, unexpected {} construct", root.label),
    })
}

pub(super) fn process_expr(
    expr: Expression,
    mut into: Option<ItemId>,
    env: &mut Environment,
    ctx: Context,
    parents: &[&Definitions],
) -> Result<ItemId, String> {
    let mut expr = expr;
    let item = if let Some(post) = expr.others.pop() {
        process_postfix(post, expr, env, parents)?
    } else {
        let root = expr.root;
        process_root(root, &mut into, env, ctx, parents)?
    };

    if let Some(id) = into {
        env.define(id, item);
        Ok(id)
    } else if let Item::Item(id) = item {
        Ok(id)
    } else {
        let id = env.next_id();
        env.define(id, item);
        Ok(id)
    }
}
