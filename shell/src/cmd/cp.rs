use crate::error::ShellError;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Cp {
    pub targets: Vec<String>,
}

impl Cp {
    pub fn new(targets: Vec<String>) -> Self {
        Self { targets }
    }

    pub fn execute(&self) -> Result<(), ShellError> {
        if self.targets.len() < 2 {
            return Err(ShellError::Other("cp: missing file operand".into()));
        }

        let dest = &self.targets[self.targets.len() - 1];
            let mut seen = HashSet::new();
            let mut duplicates = String::new();
        if self.targets.len() > 2 {
            if !Path::new(dest).is_dir() {
                return Err(ShellError::Other(format!(
                    "cp: target '{}' is not a directory",
                    dest
                ).into()));
            }

            for file in &self.targets[..self.targets.len() - 1] {
                if Path::new(file).is_file() {
                    if !seen.insert(file) {
                        duplicates.push_str(file)
                    }
                    let target_path = Path::new(dest).join(Path::new(file).file_name().unwrap());
                    fs::copy(file, target_path)?;
                } else {
                    println!("cp: '{}' is not a file", file);
                }
            }

            if !duplicates.is_empty() {
                 return Err(ShellError::Other(format!("cp: warning: source file '{}' specified more than once", duplicates).into()));
            }
            
        } else {
    let src_path = Path::new(&self.targets[0]);
    let dest_path = Path::new(dest);

    // source and destination are the same file
    if src_path == dest_path {
        return Err(ShellError::Other(format!(
            "cp: '{}' and '{}' are the same file",
            src_path.display(),
            dest_path.display()
        ).into()));
    }

    // source must be a file
    if !src_path.is_file() {
        return Err(ShellError::Other(format!(
            "cp: '{}' is not a file",
            src_path.display()
        ).into()));
    }

    // if destination is a directory, copy inside it using source filename
    let final_dest = if dest_path.is_dir() {
        dest_path.join(src_path.file_name().unwrap())
    } else {
        dest_path.to_path_buf()
    };

    fs::copy(src_path, final_dest)?;
}

        Ok(())
    }
}