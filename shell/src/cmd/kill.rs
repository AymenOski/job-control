use crate::error::ShellError;
use std::io;

#[derive(Debug, Clone)]

pub struct Kill {
    pub args: Vec<String>,
}


impl Kill {
    pub fn new(args: Vec<String>) -> Self {
        Self {args}

    }

    fn parse_signal(arg: &str) -> Result<i32, String> {
        if !arg.starts_with('-') || arg.len() == 1 {
            return Ok(libc::SIGTERM);
        }

        arg[1..]
            .parse::<i32>()
            .map_err(|_| format!("kill: invalid signal '{}'", arg))
    }

    fn parse_pid(arg: &str) -> Result<libc::pid_t, String> {
        arg.parse::<libc::pid_t>()
            .map_err(|_| format!("kill: invalid pid '{}'", arg))
    }

    pub fn execute(&self) -> Result<(), ShellError> {
        if self.args.is_empty() {
            eprintln!("kill: usage: kill [-signal] pid");
            return Ok(());
        }

        let mut signal = libc::SIGTERM;
        let mut pid_arg_index = 0;

        if self.args[0].starts_with('-') {
            signal = match Self::parse_signal(&self.args[0]) {
                Ok(sig) => sig,
                Err(msg) => {
                    eprintln!("{}", msg);
                    return Ok(());
                }
            };
            pid_arg_index = 1;
        }

        if pid_arg_index >= self.args.len() {
            eprintln!("kill: missing pid");
            return Ok(());
        }

        let pid = match Self::parse_pid(&self.args[pid_arg_index]) {
            Ok(pid) => pid,
            Err(msg) => {
                eprintln!("{}", msg);
                return Ok(());
            }
        };

        unsafe {
            if libc::kill(pid, signal) == -1 {
                eprintln!("kill: {}", io::Error::last_os_error());
            }
        }
        Ok(())
    }
}