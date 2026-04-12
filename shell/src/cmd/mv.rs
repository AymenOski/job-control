use crate::error::ShellError;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Mv {
    pub targets: Vec<String>,
}

impl Mv {
    pub fn new(args: Vec<String>) -> Self {
        Self { targets: args }
    }

    pub fn execute(&self) -> Result<(), ShellError> {
        if self.targets.len() < 2 { println!("mv: missing file or destination"); return Ok(());}

        let destination = Path::new(&self.targets[self.targets.len() - 1]);

        if self.targets.len() > 2 && !(destination.exists() && destination.is_dir()) {
            println!("mv: target '{}' is not a directory", destination.display());
            return Ok(());
        }
        if  Path::new(&self.targets[0]).is_file() &&  self.targets[0] == format!("{}", destination.display()) { println!("mv: {} and {} are the same file", self.targets[0], self.targets[0]); return Ok(());  }

        for source in &self.targets[..self.targets.len() - 1] {
            let source_path = Path::new(source);

            let final_dest = if destination.exists() && destination.is_dir() { // move 
                destination.join(source_path.file_name().unwrap())
            }else { destination.to_path_buf() }; //rename

            if let Err(_) = fs::rename(source_path, &final_dest) {
                println!("mv: cannot move '{}' to '{}'", source, final_dest.display() );
            }
        }

        Ok(())
    }
}