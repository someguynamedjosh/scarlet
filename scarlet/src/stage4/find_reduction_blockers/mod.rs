use super::structure::Environment;
use crate::shared::{Item, ItemId};

pub fn find_reduction_blockers(_env: &mut Environment) -> Result<(), String> {
    Ok(())
}

impl Environment {
    fn find_reduction_blockers(&mut self, of: ItemId, visited: &[ItemId]) -> Result<(), String> {
        if visited.contains(&of) {
            Ok(())
        } else {
            let contained_items = match &self.get(of).definition {
                Item::BuiltinOperation(op) => op.inputs(),
                // We do not need to recurse into definitions because they do not change the value
                // of this item unless they are explicitly used later.
                Item::Defining { base, .. } => vec![*base],
                Item::FromType { base, values } => [vec![*base], values.clone()].concat(),
                Item::GodType => vec![],
                Item::Pick {
                    initial_clause,
                    elif_clauses,
                    else_clause,
                } => {
                    let mut items = vec![initial_clause.0, initial_clause.1, *else_clause];
                    for (a, b) in elif_clauses {
                        items.push(*a);
                        items.push(*b);
                    }
                    items
                }
                Item::PrimitiveType(..) | Item::PrimitiveValue(..) => vec![],
                Item::Replacing {
                    base, replacements, ..
                } => std::iter::once(*base)
                    .chain(replacements.iter().cloned().map(|(_, i)| i))
                    .collect(),
                Item::TypeIs { base, typee, .. } => vec![*base, *typee],
                Item::Variable { typee, .. } => vec![*typee],
                Item::VariantInstance { typee, values, .. } => {
                    [values.clone(), vec![*typee]].concat()
                }
            };
            let new_visited = [Vec::from(visited), vec![of]].concat();
            for item in contained_items {
                self.find_reduction_blockers(item, &new_visited[..])?;
            }
            Ok(())
        }
    }
}
