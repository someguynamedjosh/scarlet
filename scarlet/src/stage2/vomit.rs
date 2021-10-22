use super::structure::{Environment, ItemId, StructField};
use crate::stage2::structure::Definition;

type Parent<'x> = (ItemId<'x>, String);
type Parents<'x> = Vec<Parent<'x>>;
type Path<'x> = Vec<Parent<'x>>;

impl<'x> Environment<'x> {
    pub fn get_name(&self, of: ItemId<'x>, context: ItemId<'x>) -> String {
        println!("{:#?}", self.get_paths(of));
        todo!()
    }

    fn get_parents(&self, of: ItemId<'x>) -> Parents<'x> {
        let mut parents = Parents::new();
        for (candidate_id, candidate) in &self.items {
            if let Definition::Struct(fields) = candidate.definition.as_ref().unwrap() {
                note_occurences_of_item(&mut parents, of, candidate_id, &fields[..]);
            }
        }
        parents
    }

    fn get_paths(&self, item: ItemId<'x>) -> Vec<Path<'x>> {
        let mut result = vec![];
        for parent in self.get_parents(item) {
            for path in self.get_paths(parent.0) {
                result.push([path, vec![parent.clone()]].concat());
            }
        }
        result
    }
}

fn note_occurences_of_item<'x>(
    parents: &mut Parents<'x>,
    item: ItemId<'x>,
    struct_id: ItemId<'x>,
    fields: &[StructField],
) {
    let mut index = 0;
    for field in fields {
        if field.value == item {
            let name = field_name(field, index);
            parents.push((struct_id, name))
        }
        index += 1;
    }
}

fn field_name(field: &StructField, index: i32) -> String {
    let name = field.name.clone().unwrap_or(format!("{}", index));
    name
}
