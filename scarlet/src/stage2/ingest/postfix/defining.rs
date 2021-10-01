use crate::{
    stage1::structure::{construct::Construct, expression::Expression, statement::Statement},
    stage2::{
        self,
        structure::{
            BuiltinOperation, BuiltinValue, Definitions, Environment, Item, Namespace, NamespaceId,
            Replacements, Value, Variable, Variant,
        },
    },
};

pub fn ingest(
    env: &mut Environment,
    remainder: Expression,
    post: Construct,
    in_namespace: NamespaceId,
) -> Item {
    let this_ns = env.new_undefined_namespace();
    let base = stage2::ingest(env, remainder, this_ns);
    let mut definitions = Definitions::new();
    for statement in post.expect_statements("defining").expect("TODO error") {
        match statement {
            Statement::Is(is) => {
                let name = is.name.expect_ident().expect("TODO error").to_owned();
                let item = stage2::ingest(env, is.value.clone(), this_ns);
                definitions.insert_no_replace(name, item);
            }
            _ => todo!("nice error"),
        }
    }
    env.define_namespace(
        this_ns,
        Namespace::Defining {
            base: base.namespace,
            definitions,
            parent: in_namespace,
        },
    );
    Item {
        namespace: this_ns,
        value: base.value,
    }
}
