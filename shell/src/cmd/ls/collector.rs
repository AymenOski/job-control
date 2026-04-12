use crate::error::ShellError;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn classify_targets(paths: Vec<PathBuf>) -> (Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>) {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let mut errors = Vec::new();
    
    for path in paths {
        match fs::symlink_metadata(&path) {
            Ok(meta) => {
                if meta.is_dir() {
                    dirs.push(path);
                } else {
                    files.push(path);
                }
            }
            Err(_) => errors.push(path),
        }
    }

    files.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));
    dirs.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));
    errors.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));

    (files, dirs, errors)
}

pub fn collect_directory_entries(dir: &Path, show_hidden: bool) -> Result<Vec<PathBuf>, ShellError> {
    let mut entries = Vec::new();

    if show_hidden {
        entries.push(PathBuf::from("."));
        entries.push(PathBuf::from(".."));
    }

    let read_dir = fs::read_dir(dir).map_err(|e| {
        ShellError::Other(format!("ls: cannot access '{}': {}", dir.display(), e))
    })?;

    for entry in read_dir {
        let entry = entry.map_err(|e| ShellError::Other(e.to_string()))?;
        let name = entry.file_name();

        if !show_hidden {
            if let Some(s) = name.to_str() {
                if s.starts_with('.') {
                    continue;
                }
            }
        }

        entries.push(entry.path());
    }

    entries.sort_by(|a, b| {
        let a_name = a.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let b_name = b.file_name().and_then(|s| s.to_str()).unwrap_or("");

        let a_special = a_name == "." || a_name == "..";
        let b_special = b_name == "." || b_name == "..";

        
        match (a_special, b_special) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a_name.to_lowercase().cmp(&b_name.to_lowercase()),
        }
    });

    Ok(entries)
}