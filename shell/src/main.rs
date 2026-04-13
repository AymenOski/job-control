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
// use std::fs;
// use std::io;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut last_dir = current_dir().unwrap_or_else(|_| dirs::home_dir().unwrap());

    loop {
        // safe current directory for prompt
        let cwd = match current_dir() {
            Ok(dir) => { last_dir = dir.clone(); dir}
            Err(_) => last_dir.clone(),
        };

        let display_dir = cwd.to_string_lossy().replace(&var("HOME").unwrap_or_default(), "~");

        // read command line
        let input = match rl.readline(&format!("\x1b[32m127.0.0.1@z01:\x1b[0m{}$ ", format!("\x1b[34m{}\x1b[0m", display_dir))) {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => {continue;}
                Err(ReadlineError::Eof) => { println!("exit"); break;}
            Err(err) => { println!("error: {}", err); break; }
        };


        let input = input.trim();
        if input.is_empty() { continue;}

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
                parts[last_idx] = &parts[last_idx][..parts[last_idx].len()-1];
            }
        }

        if parts.is_empty() { continue; }

        let cmd = parts[0].trim_matches(|c| c == '"');
        let args: Vec<String> = parts[1..].iter().map(|c| c.to_string()).collect();

        match cmd {

            "exit" => break,

            "echo" => {
                let output: Vec<String> = args.iter().map(|s| {
                    s.trim_matches(|c| c == '"' || c == '\'').to_string()
                }).collect();
                println!("{}", output.join(" "));
            }

            "pwd" => match current_dir() {
                Ok(dir) => println!("{}", dir.display()),
                Err(_) => println!("pwd: current directory not found"),
            },

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
                    if args.len() != 1 { println!("cd: too many arguments"); continue;}
                    std::path::PathBuf::from(&args[0])
                };
                
                if let Err(e) = set_current_dir(&new_dir) { println!("cd: {}: {}", new_dir.display(), e) }
            }
            
            "clear" => {
                println!("\x1B[2J\x1B[H");
                 stdout().flush().unwrap();
            }
                
            "help" => print_help(),
            
            _ => {
                
                let c_cmd = CString::new(cmd).unwrap();
                let c_args: Vec<CString> = parts.iter()
                    .map(|&s| CString::new(s).unwrap())
                    .collect();
                
                let mut arg_ptrs: Vec<*const libc::c_char> = c_args.iter()
                    .map(|s| s.as_ptr())
                    .collect();
                arg_ptrs.push(std::ptr::null());

                unsafe {
                    let pid = libc::fork();
                    if pid == 0 {
                        libc::execvp(c_cmd.as_ptr(), arg_ptrs.as_ptr());
                        eprintln!("{}: command not found", cmd);
                        libc::_exit(1);
                    } else if pid > 0 {
                        if is_background {
                            println!("[launched] {}", pid);
                        } else {
                            let mut status = 0;
                            libc::waitpid(pid, &mut status, 0);
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
            while libc::waitpid(-1, &mut status, libc::WNOHANG) > 0 {}
        }
    }
}