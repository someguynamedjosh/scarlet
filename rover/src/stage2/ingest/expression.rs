use super::context::Context;
use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::{
        ingest::{
            context::LocalInfo,
            statements::{process_definitions, process_replacements},
        },
        structure::{Item, ItemId, PrimitiveValue},
    },
};

fn ingest_from_construct(
    ctx: &mut Context,
    base_id: ItemId,
    from: Construct,
) -> Result<Item, String> {
    let statements = from.expect_statements("From").unwrap().to_owned();
    let mut vars = Vec::new();
    let mut named_vars = Vec::new();
    for statement in statements {
        match statement {
            Statement::Expression(expr) => {
                let var = ingest_expression(&mut ctx.child(), expr)?;
                vars.push(var);
            }
            Statement::Is(is) => {
                let name = is.name.expect_ident_owned()?;
                let expr = is.value;
                let var = ingest_expression(&mut ctx.child(), expr)?;
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
        let base_id = ctx.environment.insert(base_item);
        let definitions = named_vars;
        Item::Defining {
            base: base_id,
            definitions,
        }
    })
}

fn ingest_variant_construct(
    ctx: &mut Context,
    variant_name: String,
    typee_id: ItemId,
) -> Result<Item, String> {
    let (base_id, where_body, vars) = match ctx.environment.definition_of(typee_id) {
        None => (typee_id, None, vec![]),
        Some(Item::Defining { base, definitions }) => {
            let vars = match ctx.environment.definition_of(*base).as_ref().unwrap() {
                Item::FromType { vars, .. } => vars.clone(),
                _ => unreachable!(),
            };
            (*base, Some(definitions), vars)
        }
        Some(Item::FromType { vars, base }) => (*base, None, vars.clone()),
        _ => unreachable!(),
    };
    if let LocalInfo::Type(typee) = ctx.local_info {
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
            let base_id = ctx.environment.insert(base);
            Item::Defining {
                base: base_id,
                definitions,
            }
        } else {
            base
        })
    } else {
        todo!("nice error, variant used outside of Type construct")
    }
}

fn ingest_postfix_construct(
    ctx: &mut Context,
    post: Construct,
    remainder: Expression,
) -> Result<Item, String> {
    if post.label == "defining" {
        let statements = post.expect_statements("defining")?.to_owned();
        let body = process_definitions(&mut ctx.child(), statements, vec![])?;
        let mut child_ctx = ctx.child().with_additional_scope(&body);
        let base_id = ingest_expression(&mut child_ctx, remainder)?;
        Ok(Item::Defining {
            base: base_id,
            definitions: body,
        })
    } else {
        let base_id = ingest_expression(&mut ctx.child(), remainder)?;
        Ok(match &post.label[..] {
            "replacing" => {
                let statements = post.expect_statements("replacing")?.to_owned();
                let (replacements, unlabeled_replacements) =
                    process_replacements(&mut ctx.child(), statements)?;
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
            "From" => ingest_from_construct(&mut ctx.child(), base_id, post)?,
            "is_variant" => {
                let other = post.expect_single_expression("is_variant")?;
                let other = ingest_expression(&mut ctx.child(), other.clone())?;
                Item::IsSameVariant {
                    base: base_id,
                    other,
                }
            }
            "type_is" => {
                let typee = post.expect_single_expression("type_is")?;
                let typee = ingest_expression(&mut ctx.child(), typee.clone())?;
                Item::TypeIs {
                    exact: false,
                    base: base_id,
                    typee,
                }
            }
            "type_is_exactly" => {
                let typee = post.expect_single_expression("type_is_exactly")?;
                let typee = ingest_expression(&mut ctx.child(), typee.clone())?;
                Item::TypeIs {
                    exact: true,
                    base: base_id,
                    typee,
                }
            }
            _ => unreachable!(),
        })
    }
}

fn type_self(ctx: &mut Context) -> (ItemId, (String, ItemId)) {
    let self_id = ctx.get_or_create_current_id();
    let self_def = (format!("Self"), self_id);
    (self_id, self_def)
}

fn ingest_type_construct(ctx: &mut Context, root: Construct) -> Result<Item, String> {
    let statements = root.expect_statements("Type").unwrap().to_owned();
    let (self_id, self_def) = type_self(ctx);
    let inner_type = Item::InductiveType(self_id);
    let inner_type_id = ctx.environment.insert(inner_type);

    let definitions = process_definitions(ctx, statements, vec![self_def])?;
    Ok(Item::Defining {
        base: inner_type_id,
        definitions,
    })
}

fn ingest_pick_construct(ctx: &mut Context, root: Construct) -> Result<Item, String> {
    let statements = root.expect_statements("pick").unwrap();
    if statements.len() < 2 {
        todo!("nice error, pick must have at least 2 clauses.");
    }

    let initial_clause = if let Statement::PickIf(s) = &statements[0] {
        (
            ingest_expression(&mut ctx.child(), s.condition.clone())?,
            ingest_expression(&mut ctx.child(), s.value.clone())?,
        )
    } else {
        todo!("nice error, first clause must be an if.");
    };

    let last = statements.len() - 1;
    let else_clause = if let Statement::Else(s) = &statements[last] {
        ingest_expression(&mut ctx.child(), s.value.clone())?
    } else {
        todo!("nice error, first clause must be an if.");
    };

    let mut elif_clauses = Vec::new();
    for other in &statements[1..last] {
        if let Statement::PickElif(s) = other {
            elif_clauses.push((
                ingest_expression(&mut ctx.child(), s.condition.clone())?,
                ingest_expression(&mut ctx.child(), s.value.clone())?,
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

fn resolve_ident(ctx: &Context, ident: &str) -> Result<ItemId, String> {
    // Reverse to earch the closest parents first.
    for scope in ctx.parent_scopes.iter().rev() {
        for (name, val) in *scope {
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

fn ingest_identifier(ctx: &mut Context, root: Construct) -> Result<Item, String> {
    let ident = root.expect_ident()?;
    let resolved = resolve_ident(ctx, ident)?;
    Ok(Item::Item(resolved))
}

fn ingest_any_construct(ctx: &mut Context, root: Construct) -> Result<Item, String> {
    let typ_expr = root.expect_single_expression("any")?.clone();
    let typee = ingest_expression(&mut ctx.child(), typ_expr)?;
    let selff = ctx.get_or_create_current_id();
    Ok(Item::Variable { selff, typee })
}

fn ingest_i32_construct(ctx: &mut Context, root: Construct) -> Result<Item, String> {
    let val = root.expect_text("i32")?;
    let val: i32 = val.parse().unwrap();
    Ok(Item::PrimitiveValue(PrimitiveValue::I32(val)))
}

fn ingest_variant_construct_wrapper(ctx: &mut Context, root: Construct) -> Result<Item, String> {
    let def_expr = root.expect_single_expression("variant")?;
    let variant_name = def_expr.root.expect_ident()?.to_owned();
    if def_expr.others.len() != 1 {
        todo!("nice error");
    }
    let type_expr = def_expr.others[0]
        .expect_single_expression("type_is")?
        .clone();
    let typee = ingest_expression(&mut ctx.child(), type_expr)?;
    ingest_variant_construct(ctx, variant_name, typee)
}

fn ingest_root_construct(ctx: &mut Context, root: Construct) -> Result<Item, String> {
    match &root.label[..] {
        "identifier" => ingest_identifier(ctx, root),
        "Type" => ingest_type_construct(ctx, root),
        "any" => ingest_any_construct(ctx, root),
        "the" => todo!(),
        "i32" => ingest_i32_construct(ctx, root),
        "variant" => ingest_variant_construct_wrapper(ctx, root),
        "pick" => ingest_pick_construct(&mut ctx.child(), root),
        _ => todo!("nice error, unexpected {} construct", root.label),
    }
}

fn convert_expression_to_item(ctx: &mut Context, mut expr: Expression) -> Result<Item, String> {
    if let Some(post) = expr.others.pop() {
        ingest_postfix_construct(ctx, post, expr)
    } else {
        let root = expr.root;
        ingest_root_construct(ctx, root)
    }
}

fn define_or_dereference_item(ctx: &mut Context, item: Item) -> ItemId {
    if let Some(id) = ctx.current_item_id {
        ctx.environment.define(id, item);
        id
    } else if let Item::Item(id) = item {
        id
    } else {
        ctx.environment.insert(item)
    }
}

pub(super) fn ingest_expression(ctx: &mut Context, expr: Expression) -> Result<ItemId, String> {
    let item = convert_expression_to_item(ctx, expr)?;
    Ok(define_or_dereference_item(ctx, item))
}
