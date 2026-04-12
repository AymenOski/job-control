pub mod collector;
pub mod formatter;
pub mod metadata;
pub mod output;
pub mod parser;
pub mod types;

use crate::{cmd::Command, error::ShellError};

use collector::{classify_targets, collect_directory_entries};
use metadata::build_long_entry;
use output::{print_long_entries, print_simple_names};
use parser::parse_args;
use std::path::PathBuf;
pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self, args: Vec<String>) -> Result<(), ShellError> {
        let (flags, mut targets) = parse_args(&args)?;
        
        if targets.is_empty() {
            targets.push(PathBuf::from("."));
        }

        let (files, dirs, errors) = classify_targets(targets);

        for err in errors {
            println!("ls: cannot access '{}': No such file or directory", err.display());
        }

        let show_header = files.len() + dirs.len() > 1;

        if !files.is_empty() {
            if flags.l {
                let mut entries = Vec::new();
                for file in &files {
                    let entry = build_long_entry(file, &flags)?;
                    entries.push(entry);
                }
                print_long_entries(&entries, false);
            } else {
                let mut names = Vec::new();
                for file in &files {
                    names.push(
                        file.file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or(file.to_string_lossy().as_ref())
                            .to_string(),
                    );
                }
                print_simple_names(&names);
            }

            if !dirs.is_empty() {
                println!();
            }
        }

        for (i, dir) in dirs.iter().enumerate() {
            if show_header {
                println!("{}:", dir.display());
            }

            let entries = collect_directory_entries(dir, flags.a)?;

            if flags.l {
                let mut long_entries = Vec::new();
                for path in &entries {
                    long_entries.push(build_long_entry(path, &flags)?);
                }
                print_long_entries(&long_entries, true);
            } else {
                let mut names = Vec::new();
                for path in &entries {
                    names.push(
                        path.file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or(path.to_string_lossy().as_ref())
                            .to_string(),
                    );
                }
                print_simple_names(&names);
            }

            if i + 1 < dirs.len() {
                println!();
            }
        }

        Ok(())
    }
}