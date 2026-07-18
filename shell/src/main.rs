

mod cmd;
mod error;
use cmd::cp::Cp;
use cmd::cat::Cat;
use cmd::mkdir::Mkdir;
use cmd::mv::Mv;
use cmd::rm::Rm;
use cmd::help::*;
use cmd::ls::LsCommand;
use cmd::Command;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::env::*;
use std::ffi::CString;
use std::io::*;
use crate::cmd::kill::Kill;
use crate::cmd::jobs::Jobs;
use crate::cmd::bg::Bg;
use crate::cmd::fg::Fg;

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum JobStatus {
    Running,
    Stopped,
    // we need to add more states like suspended and terminated (ctr +z && ctr +c)
}

#[derive(Clone, Debug)]
struct Job {
    id: usize,
    pid: i32,
    command: String,
    status: JobStatus, // Exited || Crashed || Stopped by Ctrl+Z
}

    fn main() {
    unsafe {
        // Shell puts itself in its own process group
        // libc::setpgid(0, 0); // pid=0 means self, pgid=0 means use pid as pgid

        // Shell ignores these signals so it doesn't die
        libc::signal(libc::SIGINT, libc::SIG_IGN); // Ignore Ctrl+C
        libc::signal(libc::SIGTSTP, libc::SIG_IGN); // Ignore Ctrl+Z
        libc::signal(libc::SIGTTOU, libc::SIG_IGN);
        libc::signal(libc::SIGTTIN, libc::SIG_IGN);

        if libc::isatty(libc::STDIN_FILENO) != 0 {
        let shell_pid = libc::getpid();
        libc::setpgid(shell_pid, shell_pid);
        libc::tcsetpgrp(libc::STDIN_FILENO, shell_pid);
        }
    }

    let mut rl = DefaultEditor::new().unwrap();
    let mut last_dir = current_dir().unwrap_or_else(|_| dirs::home_dir().unwrap());
    let mut jobs: Vec<Job> = Vec::new();

    loop {
        // safe current directory for prompt
        let cwd = match current_dir() {
            Ok(dir) => {
                last_dir = dir.clone();
                dir
            }
            Err(_) => last_dir.clone(),
        };

        let display_dir = cwd.to_string_lossy().replace(&var("HOME").unwrap_or_default(), "~");

        // read command line
        let input = match
            rl.readline(
                &format!(
                    "\x1b[32m127.0.0.1@z01:\x1b[0m{}$ ",
                    format!("\x1b[34m{}\x1b[0m", display_dir)
                )
            )
        {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                println!("error: {}", err);
                break;
            }
        };

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        rl.add_history_entry(input).unwrap();

        let mut parts: Vec<&str> = input.split_whitespace().collect();

        // Detect background flag '&'
        let mut is_background = false;
        if let Some(&last) = parts.last() {
            if last == "&" {
                is_background = true;
                parts.pop();
            } else if last.ends_with('&') {
                is_background = true;
                let last_idx = parts.len() - 1;
                parts[last_idx] = &parts[last_idx][..parts[last_idx].len() - 1];
            }
        }

        if parts.is_empty() {
            continue;
        }

        let cmd = parts[0].trim_matches(|c| c == '"');
        let args: Vec<String> = parts[1..]
            .iter()
            .map(|c| c.to_string())
            .collect();

        match cmd {
            "exit" => {
                break;
            }

            "echo" => {
                let output: Vec<String> = args
                    .iter()
                    .map(|s| { s.trim_matches(|c| c == '"' || c == '\'').to_string() })
                    .collect();
                println!("{}", output.join(" "));
            }

            "pwd" =>
                match current_dir() {
                    Ok(dir) => println!("{}", dir.display()),
                    Err(_) => println!("pwd: current directory not found"),
                }

            "cat" => {
                if let Err(e) = Cat::new(args.clone()).execute(&mut rl) {
                    eprintln!("{}", e);
                }
            }

            "ls" => {
                let command = LsCommand;
                if let Err(e) = command.execute(args.clone()) {
                    eprintln!("{}", e);
                }
            }

            "mv" => {
                if let Err(e) = Mv::new(args.clone()).execute() {
                    eprintln!("{}", e);
                }
            }

            "kill" => {
                if let Err(e) = Kill::new(args.clone(), &mut jobs).execute() {
                    eprintln!("{:?}", e);
                }
            }

            "cp" => {
                if let Err(e) = Cp::new(args.clone()).execute() {
                    eprintln!("{}", e);
                }
            }

            "rm" => {
                if let Err(e) = Rm::new(args.clone()).execute() {
                    eprintln!("{}", e);
                }
            }

            "mkdir" => {
                if let Err(e) = Mkdir::new(args.clone(), cwd.clone()).execute() {
                    eprintln!("{}", e);
                }
            }

            "cd" => {
                let new_dir = if args.is_empty() {
                    dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"))
                } else {
                    if args.len() != 1 {
                        println!("cd: too many arguments");
                        continue;
                    }
                    std::path::PathBuf::from(&args[0])
                };

                if let Err(e) = set_current_dir(&new_dir) {
                    println!("cd: {}: {}", new_dir.display(), e);
                }
            }

            "clear" => {
                println!("\x1B[2J\x1B[H");
                stdout().flush().unwrap();
            }

            "help" => print_help(),

            "jobs" => {
                if let Err(e) = Jobs::new(args.clone(), &jobs).execute() {
                    eprintln!("{e}");
                }
            }

            "fg" => {
                if let Err(e) = Fg::new(args.clone(), &mut jobs).execute() {
                    eprintln!("{:?}", e);
                }
            }

            "bg" => {
                if let Err(e) = Bg::new(args.clone(), &mut jobs).execute() {
                    eprintln!("{:?}", e);
                }
            }
            
            _ => {
                let c_cmd = CString::new(cmd).unwrap();
                let c_args: Vec<CString> = parts
                    .iter()
                    .map(|&s| CString::new(s).unwrap())
                    .collect();

                let mut arg_ptrs: Vec<*const libc::c_char> = c_args
                    .iter()
                    .map(|s| s.as_ptr())
                    .collect();
                arg_ptrs.push(std::ptr::null());

                unsafe {
                    let pid = libc::fork();

                    if pid == 0 {
                        // Child: create its own process group
                        libc::setpgid(0, 0); // pid=0 means self, pgid=0 means use pid as pgid

                        // Restore default signal behavior so it CAN be interrupted
                        libc::signal(libc::SIGINT, libc::SIG_DFL);
                        libc::signal(libc::SIGTSTP, libc::SIG_DFL);

                        // Execute command
                        libc::execvp(c_cmd.as_ptr(), arg_ptrs.as_ptr());
                        eprintln!("{}: command not found", cmd);
                        libc::_exit(1);
                    } else if pid > 0 {
                        libc::setpgid(pid, pid);

                        if is_background {
                            let fresh_id = (1..).find(|id| !jobs.iter().any(|j| j.id == *id)).unwrap();
                            // Background: add to jobs, continue shell
                            let job = Job {
                                id: fresh_id,
                                pid,
                                command: input.to_string(),
                                status: JobStatus::Running,
                            };
                            println!("[{}] {}", job.id, job.pid);
                            jobs.push(job);
                        } else {
                            // Foreground: give terminal to child, wait, take it back

                            // Give terminal to child's process group
                            libc::tcsetpgrp(libc::STDIN_FILENO, pid);

                            // Wait for child (WUNTRACED so we know if it stops)
                            let mut status = 0;
                            libc::waitpid(pid, &mut status, libc::WUNTRACED);

                            let shell_pgid = libc::getpgrp();

                            libc::tcsetpgrp(libc::STDIN_FILENO, shell_pgid);

                            // Check what happened to the child process
                            if libc::WIFEXITED(status) {
                                // Child exited normally - just continue
                                
                            } else if libc::WIFSTOPPED(status) {
                                // Child was stopped by Ctrl+Z - add to jobs table

                                let fresh_id = (1..).find(|id| !jobs.iter().any(|j| j.id == *id)).unwrap();
                                let job = Job {
                                    id: fresh_id,
                                    pid,
                                    command: input.to_string(),
                                    status: JobStatus::Stopped,
                                };
                                println!("\n[{}]+  {:<25}{}", job.id, "Stopped", job.command);
                                jobs.push(job);
                            }
                        }
                    } else {
                        eprintln!("fork failed");
                    }
                }
            }
        }

        // Cleanup finished background processes (reaping zombies)
        unsafe {
            let mut status = 0;
            loop {
                let reaped_pid = libc::waitpid(-1, &mut status, libc::WNOHANG); // dont wait for the child process to finish
                if reaped_pid <= 0 {
                    break;
                }

                if let Some(pos) = jobs.iter().position(|j| j.pid == reaped_pid) {
                    jobs.remove(pos);
                }
            }
        }
    }
}
