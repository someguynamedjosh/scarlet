use std::{fs::FileType, path::Path};

#[derive(Debug, Clone)]
pub struct FileNode {
    pub self_content: String,
    pub children: Vec<(String, FileNode)>,
}

fn read_folder_contents(at: &Path) -> Vec<(String, FileNode)> {
    let mut results = Vec::new();
    for entry in std::fs::read_dir(at).unwrap() {
        let entry = entry.unwrap();
        let mut name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry
            .file_type()
            .as_ref()
            .map(FileType::is_dir)
            .unwrap_or(false);
        if !is_dir && !name.ends_with(".sr") {
            continue;
        }
        if !is_dir {
            name = name[..name.len() - 3].to_owned();
        }
        if let Some(item) = read_path(&entry.path().with_extension("")) {
            results.push((name, item))
        }
    }
    results
}

fn read_path(at: &Path) -> Option<FileNode> {
    let folder_path = at;
    let mut children = Vec::new();
    if folder_path.exists() && folder_path.is_dir() {
        children = read_folder_contents(folder_path);
    }
    let file_path = at.with_extension("sr");
    if file_path.exists() && file_path.is_file() {
        let content = std::fs::read_to_string(file_path).unwrap();
        Some(FileNode {
            self_content: content,
            children,
        })
    } else if children.len() > 0 {
        Some(FileNode {
            self_content: String::new(),
            children,
        })
    } else {
        None
    }
}

pub fn read_root<'a>(at: impl AsRef<Path>) -> Option<FileNode> {
    let root_path = at.as_ref();
    read_path(&root_path)
}
