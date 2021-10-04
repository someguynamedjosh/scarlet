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
        if name.ends_with(".sr") {
            name = name[..name.len() - 3].to_owned();
        }
        if name == "SELF" {
            continue;
        }
        if let Some(item) = read_path(&entry.path()) {
            results.push((name, item))
        }
    }
    results
}

fn read_path(at: &Path) -> Option<FileNode> {
    let sr_extension = OsString::from_str("sr").unwrap();
    let sr_extension = Some(sr_extension.as_os_str());
    if at.is_dir() && at.extension() != sr_extension {
        let self_file = at.join("SELF.sr");
        if self_file.exists() {
            Some(FileNode {
                self_def: self_file,
                children: read_folder_contents(at),
            })
        } else {
            None
        }
    } else if at.is_file() && at.extension() == sr_extension {
        Some(FileNode {
            self_def: at.to_owned(),
            children: vec![],
        })
    } else {
        None
    }
}

fn read_root(at: &Path) -> Option<FileNode> {
    let root_file = at.join("root.sr");
    let root_folder = at.join("root");
    read_path(&root_file).or(read_path(&root_folder))
}

fn parse_file_to_stage1(file: &FileNode) -> Result<Expression, String> {
    let data = std::fs::read_to_string(&file.self_def)
        .map_err(|_| format!("Failed to read {:?}", file.self_def))?;
    let (remainder, parsed) =
        stage1::ingest()(&data).map_err(|_| format!("Failed to parse {:?}", file.self_def))?;
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

    let item = Item::Defining { base, definitions };
    Ok(env.push_item(item))
}

pub fn start_from_root(path: &str) -> Result<(Environment, ItemId), String> {
    let tree = read_root(&PathBuf::from_str(path).unwrap()).unwrap();
    let mut env = Environment::new();
    let item = ingest_file_tree(&mut env, tree)?;
    Ok((env, item))
}
