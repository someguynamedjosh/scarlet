use crate::{
    parse::{
        expression::{Construct, Expression},
        statements::Statement,
    },
    stage2::{
        helpers::{expect_ident_expr, get_or_put_into, resolve_ident, Context},
        ingest::statements::{process_definitions, process_replacements},
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
            Statement::Replace(..) => todo!("nice error"),
            Statement::Is(is) => {
                let name = expect_ident_expr(is.name)?;
                let expr = is.value;
                let ctx = Context::Plain;
                let var = process_expr(expr, None, env, ctx, parents)?;
                named_vars.push((name, var));
                vars.push(var);
            }
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

fn process_recording(
    base_id: ItemId,
    as_from: Item,
    env: &mut Environment,
    ctx: Context,
    parents: &[&Definitions],
) -> Result<Item, String> {
    let (where_body, vars) = match as_from {
        Item::Defining { base, definitions } => {
            let vars = match env.definition_of(base).as_ref().unwrap() {
                Item::FromType { vars, .. } => vars.clone(),
                _ => unreachable!(),
            };
            (Some(definitions), vars)
        }
        Item::FromType { vars, .. } => (None, vars.clone()),
        _ => unreachable!(),
    };
    if let Context::TypeMember(typee, member_name) = ctx {
        if base_id != typee {
            todo!("nice error, constructor result is not Self type.")
        }
        let base = Item::InductiveValue {
            typee,
            variant_name: member_name,
            records: vars,
        };
        Ok(if let Some(definitions) = where_body {
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
        todo!("nice error, recording used outside of Type construct")
    }
}

fn process_postfix(
    post: Construct,
    remainder: Expression,
    env: &mut Environment,
    ctx: Context,
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
        "replacing" | "member" | "From" | "recording" => (),
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
            let name = expect_ident_expr(post.expect_single_expression("member")?.clone())?;
            Item::Member {
                base: base_id,
                name,
            }
        }
        "From" => {
            let statements = post.expect_statements("From")?;
            process_from(base_id, statements.to_owned(), env, parents)?
        }
        "recording" => {
            let statements = post.expect_statements("recording")?;
            let as_from = process_from(base_id, statements.to_owned(), env, parents)?;
            process_recording(base_id, as_from, env, ctx, parents)?
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
    let into = get_or_put_into(into, env);
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

fn process_root(
    root: Construct,
    into: &mut Option<ItemId>,
    env: &mut Environment,
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
            let selff = get_or_put_into(into, env);
            Item::Variable { selff, typee }
        }
        "the" => todo!(),
        "i32" => {
            let val = root.expect_text("i32")?;
            let val: i32 = val.parse().unwrap();
            Item::PrimitiveValue(PrimitiveValue::I32(val))
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
        process_postfix(post, expr, env, ctx, parents)?
    } else {
        let root = expr.root;
        process_root(root, &mut into, env, parents)?
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
