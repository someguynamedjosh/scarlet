use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    shared::{Definitions, ItemId},
    stage1,
    stage2::{self, structure::Environment},
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
        if name.ends_with(".rer") {
            name = name[..name.len() - 4].to_owned();
        }
        if name == "$" {
            continue;
        }
        if let Some(item) = read_path(&entry.path()) {
            results.push((name, item))
        }
    }
    results
}

fn read_path(at: &Path) -> Option<FileNode> {
    let rer_extension = OsString::from_str("rer").unwrap();
    let rer_extension = Some(rer_extension.as_os_str());
    if at.is_dir() && at.extension() != rer_extension {
        let self_file = at.join("$.rer");
        if self_file.exists() {
            Some(FileNode {
                self_def: self_file,
                children: read_folder_contents(at),
            })
        } else {
            None
        }
    } else if at.is_file() && at.extension() == rer_extension {
        Some(FileNode {
            self_def: at.to_owned(),
            children: vec![],
        })
    } else {
        None
    }
}

fn read_root(at: &Path) -> Option<FileNode> {
    let root_file = at.join("root.rer");
    let root_folder = at.join("root");
    read_path(&root_file).or(read_path(&root_folder))
}

fn ingest_file_tree(
    env: &mut Environment,
    tree: FileNode,
    scopes: &Vec<&Definitions>,
    into: ItemId,
) -> Result<(), String> {
    let data = std::fs::read_to_string(&tree.self_def)
        .map_err(|_| format!("Failed to read {:?}", tree.self_def))?;
    let (remainder, parsed) =
        stage1::ingest()(&data).map_err(|_| format!("Failed to parse {:?}", tree.self_def))?;
    println!("{:#?}", parsed);
    if remainder.len() > 0 {
        todo!("nice error, syntax error");
    }

    let mut child_defs = Definitions::new();
    for child in &tree.children {
        child_defs.insert_or_replace((child.0.clone(), env.next_id()));
    }
    let scopes = [scopes.clone(), vec![&child_defs]].concat();

    stage2::ingest(env, parsed, into, &scopes, child_defs.clone())?;

    for ((_, id), (_, tree)) in child_defs.iter().zip(tree.children.into_iter()) {
        ingest_file_tree(env, tree, &scopes, *id)?;
        env.set_defined_in(*id, into);
    }
    Ok(())
}

pub fn start_from_root(path: &str) -> Result<Environment, String> {
    let tree = read_root(&PathBuf::from_str(path).unwrap()).unwrap();
    let mut env = Environment::new();
    let root_id = env.next_id();
    ingest_file_tree(&mut env, tree, &vec![], root_id).unwrap();
    Ok(env)
}
