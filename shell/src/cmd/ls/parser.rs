use crate::error::ShellError;
use std::path::PathBuf;

use super::types::Flags;

pub fn parse_args(args: &[String]) -> Result<(Flags, Vec<PathBuf>), ShellError> {
    let mut flags = Flags::default();
    let mut paths = Vec::new();
    let  after_ddash = false;

    for arg in args {

        if !after_ddash && arg.starts_with('-') && arg.len() > 1 {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => flags.a = true,
                    'l' => flags.l = true,
                    'F' => flags.f = true,
                    _ => {
                        return Err(ShellError::Other(format!(
                            "ls: invalid option -- '{}'",
                            ch
                        )));
                    }
                }
            }
        } else {
            paths.push(PathBuf::from(arg));
        }
    }

    Ok((flags, paths))
}