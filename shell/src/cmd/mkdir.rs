use crate::error::ShellError;
use std::fs;
use std::path::{PathBuf};

#[derive(Debug, Clone)]
pub struct Mkdir {
    pub targets: Vec<String>,
    pub cwd: PathBuf, // current working directory
}

impl Mkdir {
    pub fn new(targets: Vec<String>, cwd: PathBuf) -> Self {
        Self { targets, cwd }
    }

    pub fn execute(&self) -> Result<(), ShellError> {
        if self.targets.is_empty() {
            return Err(ShellError::Other("mkdir: missing operand".into()));
        }

        for dir in &self.targets {
        if dir.starts_with("-") {
           return Err(ShellError::Other(format!("mkdir: Cannot create directory '{}' ", dir).into()));

        }
            let path = self.cwd.join(dir);
            if let Err(e) = fs::create_dir(&path) {
                println!("mkdir: cannot create directory '{}': {}", dir, e);
            }
        }

        Ok(())
    }
}