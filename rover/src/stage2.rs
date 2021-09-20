use crate::parse::{
    expression::{Construct, Expression},
    statements::{Is, Replace, Statement},
};
use std::fmt::{self, Debug, Formatter};

pub fn ingest(statements: Vec<Statement>) -> Result<(Environment, ItemId), String> {
    let mut env = Environment::new();
    let (rover, god_type) = define_rover_item(&mut env);
    let rover_def = (format!("rover"), rover);
    let definitions =
        process_definitions(statements, vec![rover_def], &mut env, Context::Plain, &[])?;
    let file_id = env.next_id();
    env.mark_as_module(file_id);
    env.define(
        file_id,
        Item::Defining {
            base: god_type,
            definitions,
        },
    );
    Ok((env, file_id))
}

struct UnprocessedItem {
    id: ItemId,
    public: bool,
    name: String,
    def: Expression,
}

fn expect_ident_expr(expr: Expression) -> Result<String, String> {
    if expr.others.len() > 0 {
        todo!("nice error")
    } else {
        expr.root.expect_ident().map(String::from)
    }
}

fn resolve_ident(ident: &str, parents: &[&Definitions]) -> Result<ItemId, String> {
    // Search the closest parents first.
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Context {
    Plain,
    Type(ItemId),
    TypeMember(ItemId, String),
}

fn get_or_put_into(into: &mut Option<ItemId>, env: &mut Environment) -> ItemId {
    match into {
        Some(id) => *id,
        None => {
            let id = env.next_id();
            *into = Some(id);
            id
        }
    }
}

fn process_from_construct(
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

fn process_recording_construct(
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

fn process_postfix_construct(
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
            let replacements = process_replacements(statements, env, parents)?;
            Item::Replacing {
                base: base_id,
                replacements,
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
            process_from_construct(base_id, statements.to_owned(), env, parents)?
        }
        "recording" => {
            let statements = post.expect_statements("recording")?;
            let as_from = process_from_construct(base_id, statements.to_owned(), env, parents)?;
            process_recording_construct(base_id, as_from, env, ctx, parents)?
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
    env.define(type_item, Item::Value(Value::InductiveType(into)));
    Ok(Item::Defining {
        base: type_item,
        definitions,
    })
}

fn process_root_construct(
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
        _ => todo!("nice error, unexpected {} construct", root.label),
    })
}

fn process_expr(
    expr: Expression,
    mut into: Option<ItemId>,
    env: &mut Environment,
    ctx: Context,
    parents: &[&Definitions],
) -> Result<ItemId, String> {
    let mut expr = expr;
    let item = if let Some(post) = expr.others.pop() {
        process_postfix_construct(post, expr, env, ctx, parents)?
    } else {
        let root = expr.root;
        process_root_construct(root, &mut into, env, parents)?
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

fn define_rover_item(env: &mut Environment) -> (ItemId, ItemId) {
    let god_type = env.next_id();
    env.define(god_type, Item::Value(Value::GodType));
    let i32_type = env.next_id();
    env.define(i32_type, Item::Value(Value::I32Type));
    let lang = env.next_id();
    env.mark_as_module(lang);
    env.define(
        lang,
        Item::Defining {
            base: god_type,
            definitions: vec![
                (format!("TYPE"), god_type),
                (format!("Integer32"), i32_type),
            ],
        },
    );

    let zero = env.next_id();
    env.define(zero, Item::Value(Value::I32(0)));
    let one = env.next_id();
    env.define(one, Item::Value(Value::I32(1)));
    let core = env.next_id();
    env.mark_as_module(core);
    env.define(
        core,
        Item::Defining {
            base: god_type,
            definitions: vec![(format!("zero"), zero), (format!("one"), one)],
        },
    );

    let rover = env.next_id();
    env.mark_as_module(rover);
    env.define(
        rover,
        Item::Defining {
            base: god_type,
            definitions: vec![(format!("lang"), lang), (format!("core"), core)],
        },
    );
    (rover, god_type)
}

fn process_definitions(
    statements: Vec<Statement>,
    other_defs: Vec<(String, ItemId)>,
    env: &mut Environment,
    ctx: Context,
    parents: &[&Definitions],
) -> Result<Definitions, String> {
    let mut top_level_expressions = Vec::new();
    for statement in statements {
        match statement {
            Statement::Is(is) => {
                let name = expect_ident_expr(is.name)?;
                top_level_expressions.push(UnprocessedItem {
                    id: env.next_id(),
                    public: is.public,
                    name,
                    def: is.value,
                });
            }
            Statement::Replace(s) => todo!("nice error"),
            Statement::Expression(..) => todo!("Nice error"),
        }
    }
    let definitions: Vec<_> = other_defs
        .into_iter()
        .chain(top_level_expressions.iter().map(|i| (i.name.clone(), i.id)))
        .collect();
    let parents: Vec<_> = parents
        .iter()
        .copied()
        .chain(std::iter::once(&definitions))
        .collect();
    let parents = &parents[..];
    for item in top_level_expressions {
        let next_ctx = match &ctx {
            Context::Type(type_item) => Context::TypeMember(*type_item, item.name.clone()),
            _ => Context::Plain,
        };
        process_expr(item.def, Some(item.id), env, next_ctx, parents)?;
    }
    Ok(definitions)
}

fn process_replacements(
    statements: Vec<Statement>,
    env: &mut Environment,
    parents: &[&Definitions],
) -> Result<Replacements, String> {
    let mut replacements = Replacements::new();
    for statement in statements {
        match statement {
            Statement::Is(..) => todo!("nice error"),
            Statement::Replace(s) => {
                let ctx = Context::Plain;
                let target = process_expr(s.target, None, env, ctx, parents)?;
                let ctx = Context::Plain;
                let value = process_expr(s.value, None, env, ctx, parents)?;
                replacements.push((target, value));
            }
            Statement::Expression(..) => todo!("Nice error"),
        }
    }
    Ok(replacements)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    GodType,
    I32Type,
    I32(i32),
    InductiveType(ItemId),
    InductiveValue {
        typee: ItemId,
        variant: String,
        data: (),
    },
}

impl Value {
    pub fn typ(&self) -> Self {
        match self {
            Self::GodType | Self::I32Type | Self::InductiveType(..) => Self::GodType,
            Self::I32(..) => Self::I32Type,
            Self::InductiveValue { typee, .. } => Self::InductiveType(*typee),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub(crate) usize);

impl Debug for ItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "id{{{}}}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    pub modules: Vec<ItemId>,
    pub(crate) items: Vec<Option<Item>>,
}

fn indented(source: &str) -> String {
    source.replace("\n", "\n    ")
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Environment [")?;
        for (index, item) in self.items.iter().enumerate() {
            if f.alternate() {
                write!(f, "\n\n    ")?;
            }
            write!(f, "{:?} is ", ItemId(index))?;
            match item {
                Some(item) => {
                    if f.alternate() {
                        let text = format!("{:#?}", item);
                        write!(f, "{},", indented(&text[..]))?;
                    } else {
                        write!(f, "{:?}", item)?;
                    }
                }
                None => write!(f, "None,")?,
            }
        }
        if f.alternate() {
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn mark_as_module(&mut self, item: ItemId) {
        self.modules.push(item)
    }

    pub fn next_id(&mut self) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(None);
        id
    }

    pub fn define(&mut self, item: ItemId, definition: Item) {
        assert!(item.0 < self.items.len());
        self.items[item.0] = Some(definition)
    }

    pub fn definition_of(&self, item: ItemId) -> &Option<Item> {
        assert!(item.0 < self.items.len());
        &self.items[item.0]
    }
}

pub type Definitions = Vec<(String, ItemId)>;
pub type Replacements = Vec<(ItemId, ItemId)>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Item(ItemId),
    Variable {
        selff: ItemId,
        typee: ItemId,
    },
    Value(Value),
    Member {
        base: ItemId,
        name: String,
    },
    FromType {
        base: ItemId,
        vars: Vec<ItemId>,
    },
    InductiveValue {
        typee: ItemId,
        variant_name: String,
        records: Vec<ItemId>,
    },
    Public(ItemId),
    Defining {
        base: ItemId,
        definitions: Definitions,
    },
    Replacing {
        base: ItemId,
        replacements: Replacements,
    },
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Item(id) => write!(f, "{:?}", id),
            Self::Variable { selff, typee } => write!(f, "any{{{:?}}} at {:?}", typee, selff),
            Self::Value(value) => write!(f, "{:?}", value),
            Self::Member { base, name } => write!(f, "{:?}::{}", base, name),
            Self::FromType { base, vars } => {
                write!(f, "{:?} From{{", base)?;
                if vars.len() > 0 {
                    write!(f, "{:?}", vars[0])?;
                }
                for var in &vars[1..] {
                    write!(f, " {:?}", var)?;
                }
                write!(f, "}}")
            }
            Self::InductiveValue {
                typee,
                variant_name,
                records,
            } => {
                write!(f, "inductive_value {:?}::{}[", typee, variant_name)?;
                for record in records {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{:?}, ", record)?;
                }
                if f.alternate() {
                    write!(f, "\n")?;
                }
                write!(f, "]")
            }
            Self::Public(item) => write!(f, "public {:?}", item),
            Self::Defining { base, definitions } => {
                let gap = if f.alternate() { "\n" } else { "" };
                write!(f, "{:?} {}defining{{", base, gap)?;
                for (name, def) in definitions {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{} is {:?} ", name, def)?;
                }
                write!(f, "{}}}", gap)
            }
            Self::Replacing { base, replacements } => {
                let gap = if f.alternate() { "\n" } else { "" };
                write!(f, "{:?} {}replacing{{", base, gap)?;
                for (target, value) in replacements {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{:?} with {:?} ", target, value)?;
                }
                write!(f, "{}}}", gap)
            }
        }
    }
}
