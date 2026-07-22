# Shell - Job Control

A Unix shell implementation in Rust with full job control capabilities.

## Overview

This project is a command-line shell written in Rust that implements process job control. Job control refers to the ability to selectively stop (suspend) and continue (resume) the execution of processes at a later point.

## Features

- **Job Control**
  - `jobs` - Display status of all background jobs
  - `bg` - Resume stopped jobs in the background
  - `fg` - Bring background jobs to the foreground
  - `kill` - Terminate processes by PID or job ID
  - `Ctrl+Z` - Stop foreground process

- **Built-in Commands**
  - `cd` - Change directory
  - `pwd` - Print working directory
  - `echo` - Display text
  - `ls` - List files
  - `cat` - Display file contents
  - `cp` - Copy files
  - `mv` - Move files
  - `rm` - Remove files
  - `mkdir` - Create directories
  - `help` - Display help
  - `clear` - Clear terminal
  - `exit` - Exit shell

- **Other Features**
  - Command history (up/down arrows)
  - Background process execution with `&`
  - Colored prompt with username and directory

## Installation

```bash
cd shell
cargo build
```

## Usage

```bash
cargo run
```

### Running Background Processes

```bash
tar -czf archive.tar.gz . &
sleep 50000 &
```

### Managing Jobs

```bash
jobs          # List all jobs
jobs -l       # List with PIDs
jobs -p       # Show PIDs only
jobs -r       # Show running jobs
jobs -s       # Show stopped jobs
```

### Job Control

```bash
fg %1         # Bring job 1 to foreground
bg %1         # Resume job 1 in background
kill %1       # Kill job 1
kill 12345    # Kill process with PID 12345
```

### Stopping a Process

Run a long-running command and press `Ctrl+Z` to stop it.

## Build Requirements

- Rust 2021 edition or later
- Cargo package manager

## Dependencies

- `rustyline` - Line editing and history
- `libc` - Low-level C bindings for process control
- `dirs` - Home directory detection
- `chrono` - Date/time utilities