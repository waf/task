
# Task

Command line task tracker

This is a project to help me learn [Rust](https://www.rust-lang.org/).

## Usage

- `task help` - display help documentation
- `task list` - view current tasks.
- `task add "take over the world"` - add the task "take over the world"
- `task step 1 "learn rust" - add a step to the first task
- `task complete 1a` - complete the first step (a) of the first task (1)


```shell
> ./task help

USAGE:
    task [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add         Adds a new task
    complete    Completes a task
    help        Prints this message or the help of the given subcommand(s)
    list        Lists all tasks
    step        Adds a 'step' to a task

> ./task add "take over the world"
Added task 1: take over the world

> ./task add "take over the universe"
Added task 2: take over the universe

> ./task step 1 "learn rust"
Added step "learn rust" to task "take over the world"

> ./task step 1 "write a real-world rust app"
Added step "write a real-world rust app" to task "take over the world"

> ./task list
1: take over the world
    A: learn rust
    B: write a real-world rust app
2: take over the universe

> ./task complete 1a
Completed step "learn rust"

> ./task list
1: take over the world
    A: learn rust   # rendered with strikethrough, but that's not renderable in this README 
    B: write a real-world rust app
2: take over the universe
```
