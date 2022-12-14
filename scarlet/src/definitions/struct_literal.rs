use std::fmt::{self, Formatter};

use itertools::Itertools;

use crate::item::{CddContext, CycleDetectingDebug, ItemDefinition, ItemRef};

#[derive(Clone)]
pub struct DStructLiteral<Definition, Analysis> {
    fields: Vec<(String, ItemRef<Definition, Analysis>)>,
    /// If true, a type is automatically generated based on the contents. If
    /// false, the type should be inferred.
    is_module: bool,
}

impl<Definition, Analysis> CycleDetectingDebug for DStructLiteral<Definition, Analysis> {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "[\n")?;
        for field in &self.fields {
            write!(
                f,
                "   {} IS {}",
                field.0,
                field.1.to_indented_string(ctx, 2)
            )?;
            write!(f, ",\n")?;
        }
        write!(f, "]")
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis>
    ItemDefinition<Definition, Analysis> for DStructLiteral<Definition, Analysis>
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>> {
        self.fields.iter().map(|(_, f)| f.ptr_clone()).collect_vec()
    }
}

impl<Definition, Analysis> DStructLiteral<Definition, Analysis> {
    pub fn new_module(fields: Vec<(String, ItemRef<Definition, Analysis>)>) -> Self {
        Self {
            fields,
            is_module: true,
        }
    }

    pub fn new_struct(fields: Vec<(String, ItemRef<Definition, Analysis>)>) -> Self {
        Self {
            fields,
            is_module: false,
        }
    }

    pub fn is_module(&self) -> bool {
        self.is_module
    }

    pub fn get_field(&self, name: &str) -> Option<ItemRef<Definition, Analysis>> {
        for (candidate_name, candidate_value) in &self.fields {
            if candidate_name == name {
                return Some(candidate_value.ptr_clone());
            }
        }
        None
    }
}
