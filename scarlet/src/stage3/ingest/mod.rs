use self::context::Context;
use super::structure::Environment;

mod context;
mod dereference;
mod ingest_entry;
mod ingest_structures;

pub fn ingest(input: &mut crate::stage2::structure::Environment) -> Environment {
    let mut env = Environment::new();
    let mut ctx = Context::new(input, &mut env);
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
    env
}
