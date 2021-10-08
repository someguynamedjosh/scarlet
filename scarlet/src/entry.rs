use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    stage1::{self, structure::expression::Expression},
    stage2::{
        self,
        structure::{Definitions, Environment, Item, ItemId},
    },
};

#[derive(Debug, Clone)]
struct FileNode {
    self_def: PathBuf,
    children: Vec<(String, FileNode)>,
}

fn read_folder_contents(at: &Path) -> Vec<(String, FileNode)> {
    let mut results = Vec::new();
    for entry in std::fs::read_dir(at).unwrap() {
        let entry = entry.unwrap();
        let mut name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".sr") {
            continue;
        }
        name = name[..name.len() - 3].to_owned();
        if let Some(item) = read_path(&entry.path().with_extension("")) {
            results.push((name, item))
        }
    }
    results
}

fn read_path(at: &Path) -> Option<FileNode> {
    let folder_path = at;
    let file_path = at.with_extension("sr");
    if file_path.exists() && file_path.is_file() {
        let mut children = Vec::new();
        if folder_path.exists() && folder_path.is_dir() {
            children = read_folder_contents(folder_path);
        }
        Some(FileNode {
            self_def: file_path.to_owned(),
            children,
        })
    } else {
        None
    }
}

fn read_root(at: &Path) -> Option<FileNode> {
    let root_path = at.join("root");
    read_path(&root_path)
}

fn parse_file_to_stage1(file: &FileNode) -> Result<Expression, String> {
    let data = std::fs::read_to_string(&file.self_def)
        .map_err(|_| format!("Failed to read {:?}", file.self_def))?;
    let (remainder, parsed) = stage1::ingest()(&data)
        .map_err(|err| format!("Failed to parse {:?}: {:?}", file.self_def, err))?;
    if remainder.len() > 0 {
        todo!("nice error, syntax error");
    }
    Ok(parsed)
}

fn ingest_file_tree(env: &mut Environment, tree: FileNode) -> Result<ItemId, String> {
    println!("Parsing {:?}", tree.self_def);
    let stage1_expression = parse_file_to_stage1(&tree)?;
    println!("{}", stage1::vomit(&stage1_expression));

    // This item is the actual code written in the file.
    let base = stage2::ingest_expression(env, stage1_expression);

    // Ingest all the child files.
    let mut children = Definitions::new();
    for (name, node) in tree.children {
        let item = ingest_file_tree(env, node)?;
        children.insert_no_replace(name, item);
    }
    let definitions = children;

    let definitions2 = definitions.clone();
    let item = Item::Defining { base, definitions };
    let result = env.push_item(item);
    env.set_parent_scope(base, result);
    for (_, def) in definitions2 {
        env.set_parent_scope(def, result);
    }
    Ok(result)
}

pub fn start_from_root(path: &str) -> Result<(Environment, ItemId), String> {
    let tree = read_root(&PathBuf::from_str(path).unwrap()).unwrap();
    println!("{:#?}", tree);
    let mut env = Environment::new();
    let item = ingest_file_tree(&mut env, tree)?;
    Ok((env, item))
}
