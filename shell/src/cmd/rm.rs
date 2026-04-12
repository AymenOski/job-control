use crate::error::ShellError;
pub use std::fs;
pub use std::path::Path;
#[derive(Debug, Clone)]


pub struct Rm {
    pub flag: bool,
    pub targets: Vec<String>,
}

impl Rm {
    pub fn new(args: Vec<String>) -> Self {
        let mut flag = false;
        let mut targets = Vec::new();
        for arg in args.iter() {
            if arg == "-r" || arg == "-R" {
                flag = true;
            } else if arg.starts_with('-') {
                for c in arg[1..].chars() {
                    if c != 'r' && c != 'R' {
                        println!("flag: invalid option -- '{}'", c);                       
                    }
                }

                flag = true;
            } else {
                targets.push(arg.clone());
            } 
        }
        
        Self {
            flag,
            targets,
        }
    }

    pub fn execute(&self) -> Result<() ,ShellError> {
        if self.targets.is_empty() {
           return Err(ShellError::Other("rm: missing operand".into()));
        }

        for target in &self.targets {
            self.remove_target(target)?;
        }
        Ok(())
    }

    pub fn remove_target(&self, target: &str) -> Result<(), ShellError> {
            if Self::is_protected_path(target) {
                eprintln!("rm: refusing to remove '.' or '..' : skipping '{}'", target);
                return Ok(());
            }
            let path = Path::new(target);
            let meta = match fs::symlink_metadata(path) {
                Ok(m) => m,
                Err(_) => {
                    eprintln!("rm: cannot remove '{}': No such file or directory", target);
                    return Ok(());
                }
            };

                if meta.is_symlink() || meta.is_file() {
                    fs::remove_file(path)
                        .map_err(|e| ShellError::Other(format!("rm: cannot remove '{}': {}", target, e)))?;
                        return Ok(());
                }
                if meta.is_dir() {
                    if self.flag {
                        fs::remove_dir_all(path)
                            .map_err(|e| ShellError::Other(format!("rm: cannot remove '{}': {}", target, e)))?;
                    } else {
                        eprintln!("rm: remove '{}':  Is directory", target);
                    }
                }
                Ok(())
        }

    pub fn is_protected_path(target: &str) -> bool {
            matches!(target, "." | "..")
        }


    
}