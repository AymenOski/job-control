use crate::error::ShellError;
use crate::Job;
use std::io;

#[derive(Debug)]
pub struct Kill<'a> {
    pub args: Vec<String>,
    pub jobs: &'a mut Vec<Job>,
}

impl<'a> Kill<'a> {
    pub fn new(args: Vec<String>, jobs: &'a mut Vec<Job>) -> Self {
        Self { args, jobs }
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

    pub fn execute(&mut self) -> Result<(), ShellError> {
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

        let pid_str = self.args[pid_arg_index].clone();

       
        let pid = if pid_str.starts_with('%') {
            let id: usize = match pid_str[1..].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("kill: {}: no such job", pid_str);
                    return Ok(());
                }
            };

            match self.jobs.iter().find(|j| j.id == id) {
                Some(job) => job.pid,
                None => {
                    eprintln!("kill: {}: no such job", pid_str);
                    return Ok(());
                }
            }
        } else {
            match Self::parse_pid(&pid_str) {
                Ok(p) => p,
                Err(msg) => {
                    eprintln!("{}", msg);
                    return Ok(());
                }
            }
        };


    unsafe {
    // enter kernel mode through syscall
    // Kernel resolves:
    // pid => task_struct
    // and queues the signal inside
    // process kernel state
        
    if libc::kill(pid, signal) == -1 {
        eprintln!("kill: {}", io::Error::last_os_error());
    } else {
        // remove shell side job metadata
        // real process cleanup happens
        // later inside kernel scheduler
        // and memory management subsystems

        if let Some(pos) = self.jobs.iter().position(|j| j.pid == pid) {
            let job = &self.jobs[pos];
            let indicator = if pos == self.jobs.len() - 1 { "+" } else { "-" };
            println!("[{}]{}  Terminated              {}", job.id, indicator, job.command);
            
            self.jobs.remove(pos);
            for (i, job) in self.jobs.iter_mut().enumerate() {
            job.id = i + 1;
                }
        }
    }
}
        Ok(())
    }
}
