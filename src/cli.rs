extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};
use std::ffi::OsString;

pub const ADD_COMMAND: &'static str  = "add";
pub const LIST_COMMAND: &'static str = "list";
pub const STEP_COMMAND: &'static str = "step";
pub const COMPLETE_COMMAND: &'static str = "complete";
pub const COMMAND_TARGET: &'static str = "target";
pub const COMMAND_DESCRIPTION: &'static str = "description";

pub fn parse_arguments<'a, I, T>(args: I) -> ArgMatches<'a>
    where I: IntoIterator<Item = T>,
          T: Into<OsString>
{
    return App::new("Task")
        .version("1.0")
        .author("Will Fuqua")
        .subcommand(SubCommand::with_name(ADD_COMMAND)
                    .about("Adds a new task")
                    .arg(Arg::with_name(COMMAND_DESCRIPTION)
                         .short("d")
                         .index(1)
                         .required(true)))
        .subcommand(SubCommand::with_name(LIST_COMMAND)
                    .about("Lists all tasks"))
        .subcommand(SubCommand::with_name(STEP_COMMAND)
                    .about("Adds a 'step' to a task")
                    .arg(Arg::with_name(COMMAND_TARGET)
                         .short("t")
                         .index(1)
                         .required(true))
                    .arg(Arg::with_name(COMMAND_DESCRIPTION)
                         .short("d")
                         .index(2)
                         .required(true)))
        .subcommand(SubCommand::with_name(COMPLETE_COMMAND)
                    .about("Completes a task")
                    .arg(Arg::with_name(COMMAND_TARGET)
                         .short("t")
                         .index(1)
                         .required(true)))
        .get_matches_from(args);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_command_parses() {
        let args = vec!["task", "add", "here's a new task"];
        let parsed = parse_arguments(args);
        let subcommand = parsed.subcommand_name(); 
        assert_eq!(ADD_COMMAND.to_owned(), subcommand.unwrap());
    }
}

