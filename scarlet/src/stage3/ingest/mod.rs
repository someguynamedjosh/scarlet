use self::context::Context;
use super::structure::Environment;

mod context;
mod dereference;
mod ingest_entry;
mod ingest_namespace;
mod ingest_structures;

pub fn ingest(input: &mut crate::stage2::structure::Environment) -> Environment {
    let mut env = Environment::new();
    let mut ctx = Context::new(input, &mut env);
    ingest_values(&mut ctx);
    ingest_namespaces(&mut ctx);
    env
}

fn ingest_values(ctx: &mut Context) {
    if let Some(first) = ctx.input.values.first() {
        let mut id = first;
        loop {
            ctx.ingest(id);
            id = match ctx.input.values.next(id) {
                Some(id) => id,
                None => break,
            };
        }
    }
}

fn ingest_namespaces(ctx: &mut Context) {
    if let Some(first) = ctx.input.namespaces.first() {
        let mut id = first;
        loop {
            ctx.ingest_namespace(id);
            id = match ctx.input.namespaces.next(id) {
                Some(id) => id,
                None => break,
            };
        }
    }
}
