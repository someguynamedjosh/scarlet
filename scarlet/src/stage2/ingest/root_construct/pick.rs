use crate::{
    shared::Item,
    stage1::structure::{construct::Construct, statement::Statement},
    stage2::{
        ingest::{context::Context, expression::ingest_expression},
        structure::UnresolvedItem,
    },
};

pub fn ingest_pick_construct(ctx: &mut Context, root: Construct) -> Result<UnresolvedItem, String> {
    let statements = root.expect_statements("pick").unwrap();
    if statements.len() < 1 {
        todo!("nice error, pick must have at least 1 clause.");
    }

    let last = statements.len() - 1;
    let default = if let Statement::Else(s) = &statements[last] {
        ingest_expression(&mut ctx.child(), s.value.clone(), Default::default())?
    } else {
        todo!("nice error, last clause must be an else.");
    };

    let mut clauses = vec![];

    if statements.len() > 1 {
        if let Statement::PickIf(s) = &statements[0] {
            clauses.push((
                ingest_expression(&mut ctx.child(), s.condition.clone(), Default::default())?,
                ingest_expression(&mut ctx.child(), s.value.clone(), Default::default())?,
            ));
        } else {
            todo!("nice error, first clause must be an if.");
        };

        for other in &statements[1..last] {
            if let Statement::PickElif(s) = other {
                clauses.push((
                    ingest_expression(&mut ctx.child(), s.condition.clone(), Default::default())?,
                    ingest_expression(&mut ctx.child(), s.value.clone(), Default::default())?,
                ));
            } else {
                todo!("nice error, other clauses must be elif");
            }
        }
    }

    Ok(Item::Pick { clauses, default }.into())
}
