use crate::{Job, JobStatus};

pub struct Jobs<'a> {
    pub args: Vec<String>,
    pub jobs: &'a Vec<Job>,
}

impl<'a> Jobs<'a> {
    pub fn new(args: Vec<String>, jobs: &'a Vec<Job>) -> Self {
        Self { args, jobs }
    }

    pub fn execute(&self) -> Result<(), String> {
        let (mut p_flag, mut l_flag, mut r_flag, mut s_flag) = (false, false, false, false);
        let mut jobspec: Option<usize> = None;

        // Single pass: validate and classify every argument
        for arg in &self.args {
            match arg.as_str() {
                "-p" => p_flag = true,
                "-l" => l_flag = true,
                "-r" => r_flag = true,
                "-s" => s_flag = true,
                _ => match arg.strip_prefix('%').and_then(|s| s.parse::<usize>().ok()) {
                    Some(n) => jobspec = Some(n),
                    None => {
                        eprintln!("jobs: {arg}: invalid option");
                        eprintln!("jobs: usage: jobs [-lprs] [%job_number]");
                        return Ok(());
                    }
                },
            }
        }

        if let Some(n) = jobspec {
            if !self.jobs.iter().any(|j| j.id == n) {
                eprintln!("jobs: %{n}: no such job");
                return Ok(());
            }
        }

        let jobs_len = self.jobs.len();
        for (idx, job) in self.jobs.iter().enumerate() {
            if jobspec.is_some_and(|n| job.id != n)      { continue; }
            if r_flag && job.status != JobStatus::Running { continue; }
            if s_flag && job.status != JobStatus::Stopped { continue; }

            let indicator = match idx + 1 {
                i if i == jobs_len     => "+",
                i if i == jobs_len - 1 => "-",
                _                      => " ",
            };

            if p_flag {
                println!("{}", job.pid);
                continue;
            }

            let status = match job.status {
                JobStatus::Running => "Running",
                JobStatus::Stopped => "Stopped",
            };

            if l_flag {
                println!("[{}]{}  {} {:<20}{}", job.id, indicator, job.pid, status, job.command);
            } else {
                println!("[{}]{}  {:<22}{}", job.id, indicator, status, job.command);
            }
        }

        Ok(())
    }
}
