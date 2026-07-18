use crate::{Job, JobStatus};

pub struct Bg<'a> {
    pub args: Vec<String>,
    pub jobs: &'a mut Vec<Job>,
}

impl<'a> Bg<'a> {
    pub fn new(args: Vec<String>, jobs: &'a mut Vec<Job>) -> Self {
        Self { args, jobs }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let target_job = if self.args.is_empty() {
            self.jobs.last().cloned()
        } else {
            let spec = &self.args[0];

            if spec.starts_with('%') {
                let id_str = &spec[1..];

                if id_str.is_empty() || id_str == "+" || id_str == "%" {
                    self.jobs.last().cloned()
                } else if id_str == "-" {
                    if self.jobs.len() > 1 {
                        Some(self.jobs[self.jobs.len() - 2].clone())
                    } else {
                        None
                    }
                } else {
                    match id_str.parse::<usize>() {
                        Ok(id) => self.jobs.iter().find(|j| j.id == id).cloned(),
                        Err(_) => None,
                    }
                }
            } else {
                match spec.parse::<usize>() {
                    Ok(id) => self.jobs.iter().find(|j| j.id == id).cloned(),
                    Err(_) => None,
                }
            }
        };

        let job = match target_job {
            Some(job) => job,
            None => {
                if self.args.is_empty() {
                    eprintln!("bg: current: no such job");
                } else {
                    eprintln!("bg: {}: no such job", self.args[0]);
                }
                return Ok(());
            }
        };

        // Update status in the jobs list to Running
        if let Some(pos) = self.jobs.iter().position(|j| j.pid == job.pid) {
            self.jobs[pos].status = JobStatus::Running;

            let indicator = match pos + 1 {
                i if i == self.jobs.len() => "+",
                i if self.jobs.len() > 1 && i == self.jobs.len() - 1 => "-",
                _ => " ",
            };

            println!("[{}]{}  {:<25}{}",job.id,indicator,"Continued",job.command);

            unsafe {
                // Send SIGCONT to the process group of the job to resume it
                libc::kill(-job.pid, libc::SIGCONT);
            }
        }

        Ok(())
    }
}