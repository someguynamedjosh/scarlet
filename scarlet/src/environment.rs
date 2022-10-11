use std::{cell::RefCell, collections::HashMap};

use crate::{
    definitions::{new_value::DNewValue, struct_literal::DStructLiteral},
    diagnostic::Diagnostic,
    item::{
        query::{Query, QueryContext, RootQuery, TypeCheckQuery, TypeQuery},
        IntoItemPtr, ItemPtr,
    },
};

thread_local! {
    pub static ENV: RefCell<Environment> = RefCell::new(Environment::new());
}

pub fn r#true() -> ItemPtr {
    ENV.with(|env| env.borrow().r#true())
}

/// This struct guarantees certain parts of the code remain internal to the
/// library without having to put them in the same module.
pub(crate) struct OnlyConstructedByEnvironment(());

#[derive(Clone)]
pub struct Environment {
    language_items: HashMap<String, ItemPtr>,
    root: ItemPtr,
    all_items: Vec<ItemPtr>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            language_items: HashMap::new(),
            root: DStructLiteral::new_module(vec![]).into_ptr(),
            all_items: vec![],
        }
    }

    pub fn define_language_item(
        &mut self,
        name: &str,
        definition: ItemPtr,
    ) -> Result<(), Diagnostic> {
        if self.language_items.contains_key(name) {
            Err(Diagnostic::new().with_text_error(format!(
                "Language item \"{}\" defined multiple times.",
                name
            )))
        } else {
            self.language_items.insert(name.to_owned(), definition);
            Ok(())
        }
    }

    pub fn get_language_item(&self, name: &str) -> Result<&ItemPtr, Diagnostic> {
        self.language_items.get(name).ok_or_else(|| {
            Diagnostic::new()
                .with_text_error(format!("The language item \"{}\" is not defined.", name))
        })
    }

    #[must_use]
    pub(crate) fn set_root(&mut self, root: ItemPtr) -> Vec<Diagnostic> {
        self.all_items.clear();
        root.set_parent_recursive(None);
        self.root = match root.query_resolved(&mut Self::root_query()) {
            Ok(root) => root,
            Err(diagnostic) => return vec![diagnostic],
        };
        self.root.collect_self_and_children(&mut self.all_items);
        let mut constraints = Vec::new();
        for item in &self.all_items {
            constraints.append(&mut item.collect_constraints());
        }
        let mut errors = vec![];
        let total = constraints.len();
        for (subject, constraint) in constraints {
            let original = constraint;
            let constraint = original.reduce(&HashMap::new());
            let success = constraint.is_true();
            if !success {
                errors.push(
                    Diagnostic::new()
                        .with_text_error(format!("Unsatisfied constraint:"))
                        .with_item_error(&original)
                        .with_text_info(format!("Constraint reduced to:"))
                        .with_item_info(&constraint)
                        .with_text_info(format!("Required by the following expression:"))
                        .with_item_info(&subject),
                )
            }
        }
        println!(
            "{} successes, {} errors",
            total - errors.len(),
            errors.len()
        );
        errors
    }

    pub fn root_query() -> QueryContext<RootQuery> {
        QueryContext::root(OnlyConstructedByEnvironment(()))
    }

    pub fn query_root_type(&self) -> <TypeQuery as Query>::Result {
        self.root.query_type(&mut Self::root_query())
    }

    pub fn query_root_type_check(&self) -> <TypeCheckQuery as Query>::Result {
        self.root.query_type_check(&mut Self::root_query())
    }

    pub fn r#true(&self) -> ItemPtr {
        DNewValue::r#true(self).unwrap().into_ptr()
    }
}
