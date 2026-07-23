use crate::error::ShellError;
use crate::Job;
use std::io;

// Represents the shell built-in `kill` command.
// Sends signals to processes or jobs identified by PID or job ID (%N).
#[derive(Debug)]
pub struct Kill<'a> {
    pub args: Vec<String>,
    pub jobs: &'a mut Vec<Job>,
}

impl<'a> Kill<'a> {

    // * `args` - Command-line arguments (e.g., "-9", "1234", "%1")
    // * `jobs` - Mutable reference to session's job list for lookup and cleanup
    pub fn new(args: Vec<String>, jobs: &'a mut Vec<Job>) -> Self {
        Self { args, jobs }
    }

    fn parse_signal(arg: &str) -> Result<i32, String> {
        // If arg doesn't start with '-' or is just '-', default to SIGTERM
        if arg.len() == 1 {
            return Ok(libc::SIGTERM);
        }

        // Parse the numeric signal value after the '-' prefix
        arg[1..]
            .parse::<i32>()
            .map_err(|_| format!("kill: invalid signal '{}'", arg))
    }

    fn parse_pid(arg: &str) -> Result<libc::pid_t, String> {
        arg.parse::<libc::pid_t>()
            .map_err(|_| format!("kill: invalid pid '{}'", arg))
    }


    pub fn execute(&mut self) -> Result<(), ShellError> {
        // Display usage message if no arguments provided
        if self.args.is_empty() {
            eprintln!("kill: usage: kill [-signal] pid");
            return Ok(());
        }

        // Default to SIGTERM signal (standard graceful termination)
        let mut signal = libc::SIGTERM;
        let mut pid_arg_index = 0;

        // Check if the first argument is a signal specifier (starts with '-')
        if self.args[0].starts_with('-') {
            signal = match Self::parse_signal(&self.args[0]) {
                Ok(sig) => sig,
                Err(msg) => {
                    eprintln!("{}", msg);
                    return Ok(());
                }
            };
            // Signal was parsed, so PID argument is at index 1
            pid_arg_index = 1;
        }

        // Ensure a PID or job ID argument was provided after the optional signal
        if pid_arg_index >= self.args.len() {
            eprintln!("kill: missing pid");
            return Ok(());
        }

        let pid_str = self.args[pid_arg_index].clone();

        // Resolve the target PID from either job ID (%N) or direct PID
        let pid = if pid_str.starts_with('%') {
            // Job ID format: %N where N is the job number
            let id: usize = match pid_str[1..].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("kill: {}: no such job", pid_str);
                    return Ok(());
                }
            };

            // Look up the job by ID to get its process ID
            match self.jobs.iter().find(|j| j.id == id) {
                Some(job) => job.pid,
                None => {
                    eprintln!("kill: {}: no such job", pid_str);
                    return Ok(());
                }
            }
        } else {
            // Direct PID format: parse the numeric process ID
            match Self::parse_pid(&pid_str) {
                Ok(p) => p,
                Err(msg) => {
                    eprintln!("{}", msg);
                    return Ok(());
                }
            }
        };

        unsafe {
            // Send the signal to the target process via the kill() system call
            if libc::kill(pid, signal) == -1 {
                // Signal delivery failed (e.g., permission denied, no such process)
                eprintln!("kill: {}", io::Error::last_os_error());
            } else {
                // Signal successfully queued — now clean up shell-side job metadata

                // Find and remove the job from the session's job list
                if let Some(pos) = self.jobs.iter().position(|j| j.pid == pid) {
                    let job = &self.jobs[pos];
                    // Determine job indicator (+ for current, - for previous)
                    let indicator = if pos == self.jobs.len() - 1 { "+" } else { "-" };
                    println!("[{}]{}  Terminated              {}", job.id, indicator, job.command);

                    // Remove the job from the list
                    self.jobs.remove(pos);
                    // Renumber remaining jobs to maintain sequential IDs (1, 2, 3, ...)
                    for (i, job) in self.jobs.iter_mut().enumerate() {
                        job.id = i + 1;
                    }
                }
            }
        }
        Ok(())
    }
}
