# Job Control Project – Team Roadmap

## Team Split

* **Aymen** → Execution / Process Layer
* **Mohammed** → Job Control Core
* **Youssef** → Builtins / User Interface

---

# Aymen — Execution / Process Layer

## Goal

Implement process launching, process groups, terminal control, and background/foreground execution.

## Roadmap

### Phase 1 — Understand Core Concepts

* PID vs PGID
* Foreground process group
* Terminal ownership
* Why shells use process groups

### Phase 2 — Background Launch (`&`)

* Detect background operator in parser
* `fork()` child
* `setpgid(0, 0)` in child
* Parent stores PGID in job table
* Print `[job_id] pid`

### Phase 3 — Foreground Launch

* Launch normal foreground process
* Transfer terminal with `tcsetpgrp()`
* Wait for foreground completion/stop
* Restore terminal to shell afterward

### Phase 4 — Ctrl+C / Ctrl+Z Compatibility

* Ensure shell does not die on Ctrl+C / Ctrl+Z
* Foreground child receives signals instead

### Deliverables

* Working foreground execution
* Working background execution
* Correct terminal ownership transfer

---

# Mohammed — Job Control Core

## Goal

Maintain all internal job state and synchronize with child process status.

## Roadmap

### Phase 1 — Job Data Structures

* Create `Job` struct
* Store:

  * Job ID
  * PGID
  * Command string
  * State

### Phase 2 — Job Table Management

* `add_job()`
* `remove_job()`
* `find_job_by_id()`
* `find_job_by_pgid()`
* Current / Previous job tracking

### Phase 3 — SIGCHLD Handling

* Install SIGCHLD handler
* Loop `waitpid(-1, ..., WNOHANG | WUNTRACED | WCONTINUED)`

### Phase 4 — State Updates

* Detect:

  * Running
  * Stopped
  * Continued
  * Exited
  * Signaled/Terminated
* Update job table accordingly

### Deliverables

* Accurate job tracking
* Real-time state synchronization
* Robust signal handling

---

# Youssef — Builtins / User Interface

## Goal

Implement all required job-control builtins.

## Roadmap

### Phase 1 — `jobs`

* Basic listing
* Format output like spec
* Show current/previous markers (`+` / `-`)

### Phase 2 — `jobs` Flags

* `jobs -l`
* `jobs -p`
* `jobs -r`
* `jobs -s`

### Phase 3 — `fg`

* Resolve target job
* Move to foreground
* Resume stopped jobs with SIGCONT

### Phase 4 — `bg`

* Resolve target stopped job
* Resume with SIGCONT in background
* Print resumed message

### Phase 5 — `kill`

* Kill by PID
* Kill by `%jobid`
* Display termination status

### Deliverables

* All builtins functional
* Spec-compliant output formatting

---

# Integration Plan

## Milestone 1

* Background jobs launch and appear in `jobs`

## Milestone 2

* Job states update automatically via SIGCHLD

## Milestone 3

* `fg` / `bg` work with stopped jobs

## Milestone 4

* Ctrl+Z stops foreground job correctly

## Milestone 5

* Final audit / edge-case testing

---

# Shared API Contract

```rust
add_job(...)
remove_job(...)
find_job(...)
mark_running(...)
mark_stopped(...)
mark_done(...)
get_current_job(...)
```

---

# Notes

* Merge frequently to avoid integration hell.
* Test terminal/signal behavior together early.
* Do not build builtins before the job system exists.
