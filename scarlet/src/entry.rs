use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    stage1::{self, structure::expression::Expression},
    stage2::structure::{Definitions, Environment, Scope, ScopeId},
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
    let sr_extension = OsString::from_str("sr").unwrap();
    let sr_extension = Some(sr_extension.as_os_str());
    if at.is_dir() && at.extension() != sr_extension {
        let self_file = at.join("$.sr");
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
    println!("{:#?}", parsed);
    if remainder.len() > 0 {
        todo!("nice error, syntax error");
    }
    Ok(parsed)
}

fn make_child_defs(
    env: &mut Environment,
    tree: &FileNode,
    this_file_scope: ScopeId,
) -> Definitions {
    let mut child_defs = Definitions::new();
    for child in &tree.children {
        let child_scope = env.new_undefined_item(this_file_scope);
        child_defs.insert_no_replace((child.0.clone(), child_scope));
    }
    child_defs
}

fn ingest_children(
    env: &mut Environment,
    parent: FileNode,
    parent_scope: ScopeId,
    child_items: &Definitions,
    scopes_visible_here: &Vec<&Definitions>,
) -> Result<(), String> {
    let iter = child_items.iter().zip(parent.children.into_iter());
    for ((_, scope_item), (_, child_tree)) in iter {
        let scope = Scope {
            definition: Some(*scope_item),
            parent: Some(parent_scope),
        };
        let scope = env.scopes.push(scope);
        ingest_file_tree(env, child_tree, scopes_visible_here, scope)?;
        // env.set_defined_in(*id, into);
    }
    Ok(())
}

fn ingest_file_tree(
    env: &mut Environment,
    tree: FileNode,
    parent_scopes: &Vec<&Definitions>,
    this_file_scope: ScopeId,
) -> Result<(), String> {
    let file = parse_file_to_stage1(&tree)?;
    println!("{:#?}", file);

    let children_of_file = make_child_defs(env, &tree, this_file_scope);
    let scopes_visible_here = [parent_scopes.clone(), vec![&children_of_file]].concat();

    // stage2::ingest(env, parsed, into, &scopes, child_defs.clone())?;
    ingest_children(
        env,
        tree,
        this_file_scope,
        &children_of_file,
        &scopes_visible_here,
    )?;
    Ok(())
}

pub fn start_from_root(path: &str) -> Result<Environment, String> {
    let tree = read_root(&PathBuf::from_str(path).unwrap()).unwrap();
    let mut env = Environment::new();
    let root_id = env.get_root_scope();
    ingest_file_tree(&mut env, tree, &vec![], root_id).unwrap();
    Ok(env)
}
