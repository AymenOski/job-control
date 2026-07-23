use crate::{Job, JobStatus};

// Represents the shell built-in `jobs` command.
// This command displays information about active jobs in the current session.
// It supports various flags for filtering and formatting job output.
pub struct Jobs<'a> {
    pub args: Vec<String>,
    pub jobs: &'a Vec<Job>,
}

impl<'a> Jobs<'a> {
    pub fn new(args: Vec<String>, jobs: &'a Vec<Job>) -> Self {
        Self { args, jobs }
    }

    pub fn execute(&self) -> Result<(), String> {
        // Flag variables to track command options
        let (mut p_flag, mut l_flag, mut r_flag, mut s_flag) = (false, false, false, false);
        // Optional job specification (%N) to filter a specific job
        let mut jobspec: Option<usize> = None;

        for arg in &self.args {
            match arg.as_str() {
                "-p" => p_flag = true,
                "-l" => l_flag = true,
                "-r" => r_flag = true,
                "-s" => s_flag = true,
                // Parse job specification (e.g., %1, %2) — must be %N where N is a number
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

        // Validate that the specified job actually exists in the job list.
        // If a jobspec was given but no matching job is found, report an error.
        if let Some(n) = jobspec {
            if !self.jobs.iter().any(|j| j.id == n) {
                eprintln!("jobs: %{n}: no such job");
                return Ok(());
            }
        }

        // Iterate over all jobs and display matching ones with appropriate formatting.
        let jobs_len = self.jobs.len();
        for (idx, job) in self.jobs.iter().enumerate() {
            // Filter: if a specific job was requested, skip jobs with non-matching IDs
            if jobspec.is_some_and(|n| job.id != n)      { continue; }
            // Filter: if -r flag is set, skip jobs that are not running
            if r_flag && job.status != JobStatus::Running { continue; }
            // Filter: if -s flag is set, skip jobs that are not stopped/suspended
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
                JobStatus::DonePending => "Done",
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
