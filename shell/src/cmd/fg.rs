use crate::{Job, JobStatus};

pub struct Fg<'a> {
    pub args: Vec<String>,
    pub jobs: &'a mut Vec<Job>,
}

impl<'a> Fg<'a> {
    pub fn new(args: Vec<String>, jobs: &'a mut Vec<Job>) -> Self {
        Self { args, jobs }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        // Find the job to bring to foreground
        let job_index = if self.args.is_empty() {
            // No args: use current job (last in table)
            if self.jobs.is_empty() {
                println!("bash: fg: current: no such job");
                return Ok(());
            }

            self.jobs.len() - 1
        } else {
            // Parse argument: must be in format "%1"
            let arg = &self.args[0];

            if !arg.starts_with('%') {
                println!("bash: fg: {}: no such job", arg);
                return Ok(());
            }

            let mut found: Option<usize> = None;

            // Parse job id like "%1"
            match arg[1..].parse::<usize>() {
                Ok(id) => {
                    // Find job by id
                    for (i, job) in self.jobs.iter().enumerate() {
                        if job.id == id {
                            found = Some(i);
                            break;
                        }
                    }
                }
                Err(_) => {
                    println!("bash: fg: {}: no such job", arg);
                    return Ok(());
                }
            }

            match found {
                Some(idx) => idx,
                None => {
                    println!("bash: fg: {}: no such job", arg);
                    return Ok(());
                }
            }
        };

        // Running the job in foreground after we have found the job index
        let pid = self.jobs[job_index].pid;
        let command = self.jobs[job_index].command.clone();

        // Print the command
        println!("{}", command);

        // Transfer terminal to job's process group
        unsafe {
            libc::tcsetpgrp(libc::STDIN_FILENO, pid);
        }

        // Wait for job to finish or stop (WUNTRACED catches Ctrl+Z)
        let mut status = 0;
        unsafe {
            libc::kill(-pid, libc::SIGCONT);
            libc::waitpid(pid, &mut status, libc::WUNTRACED);
        }

        // Get terminal back
        let shell_pgid = unsafe { libc::getpgrp() };
        unsafe {
            libc::tcsetpgrp(libc::STDIN_FILENO, shell_pgid);
        }

        // Check what happened to the job
        if libc::WIFEXITED(status) {
            // Job exited normally
            self.jobs.remove(job_index);

            for (i, job) in self.jobs.iter_mut().enumerate() {
                job.id = i + 1;
            }
        } else if libc::WIFSTOPPED(status) {
            // Job was stopped (Ctrl+Z)
            self.jobs[job_index].status = JobStatus::Stopped;

            println!(
                "\n[{}]+  {:<25}{}",
                self.jobs[job_index].id,
                "Stopped",
                self.jobs[job_index].command
            );
        } else if libc::WIFSIGNALED(status) {
            // Job was killed by signal
            self.jobs.remove(job_index);

            for (i, job) in self.jobs.iter_mut().enumerate() {
                job.id = i + 1;
            }
        }

        Ok(())
    }
}