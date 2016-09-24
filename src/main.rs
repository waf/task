extern crate clap;
extern crate regex;
mod cli;
mod task;
mod task_io;
use std::io::{Write, stdout, stderr};
use std::env;

fn main() {

    let input = cli::parse_arguments(&mut env::args_os());

    // read the user's tasks into memory
    let mut tasks = task_io::read_tasks().expect("could not open task store");

    // manipulate the tasks according to the command line arg
    let result = match input.subcommand() {
        (cli::LIST_COMMAND, _) => {
            task::list_tasks(&tasks)
        },
        (cli::ADD_COMMAND, Some(subcommand)) => {
            let task_title = subcommand.value_of(cli::COMMAND_DESCRIPTION).unwrap(); // unwrap is ok because arg is required by arg parser
            task::add_new_task(&mut tasks, task_title)
        },
        (cli::STEP_COMMAND, Some(subcommand)) => {
            let target = subcommand.value_of(cli::COMMAND_TARGET).unwrap(); // unwrap is ok because arg is required by arg parser
            let description = subcommand.value_of(cli::COMMAND_DESCRIPTION).unwrap();
            task::add_step_command(&mut tasks, target, description)
        },
        (cli::COMPLETE_COMMAND, Some(subcommand)) => {
            let target = subcommand.value_of(cli::COMMAND_TARGET).unwrap(); // unwrap is ok because arg is required by arg parser
            task::complete_task_or_step(&mut tasks, target)
        },
        _ => {
            Ok(input.usage().to_owned())
        }
    };

    // output result of manipulation
    match result {
        Ok(output) => writeln!(&mut stdout(), "{}", output),
        Err(error) => writeln!(&mut stderr(), "Error: {}", error),
    }.expect("failed to print to console");

    // save user's tasks to disk
    task_io::save_tasks(&tasks).expect("could not save task store");
}

