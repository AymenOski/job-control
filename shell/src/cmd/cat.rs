use crate::error::ShellError;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Cat {
    pub flag: bool,
    pub targets: Vec<String>,
}

impl Cat {
    pub fn new(args: Vec<String>) -> Self {
        let flag = args.is_empty() || (args.len() == 1 && args[0] == "-");

        let targets = if flag { Vec::new() } else { args };

        Self { flag, targets }
    }

    pub fn execute(&self, rl: &mut DefaultEditor) -> Result<(), ShellError> {
        if self.flag {
            loop {
                match rl.readline("") {
                    Ok(line) => println!("{}", line),
                    Err(ReadlineError::Interrupted) => continue, 
                    Err(ReadlineError::Eof) => break,
                    Err(_) => {
                        return Err(ShellError::Other(
                            "cat: failed to read from stdin".into(),
                        ))
                    }
                }
            }
        } else {
            for file in &self.targets {
                let path = Path::new(file);

                if path.is_dir() {
                    return Err(ShellError::Other( format!("cat: {}: Is a directory", file).into()));
                }

                match fs::read_to_string(path) {
                    Ok(content) => print!("{}", content),
                    Err(_) => {
                        return Err(ShellError::Other(format!("cat: {}: No such file or directory", file).into()))
                    }
                }
            }
        }

        Ok(())
    }
}