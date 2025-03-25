use clap::{Arg, ArgAction, Command, value_parser};

pub fn build_cli() -> Command {
    Command::new("shield")
        .version("0.1.0")
        .about("Shield CLI tool is a development framework for circom developers")
        .subcommand(build_init_command())
        .subcommand(build_delete_command())
        .subcommand(build_list_command())
}

fn build_init_command() -> Command {
    Command::new("init")
        .about("generates the boilerplate code")
        .arg(
            Arg::new("path")
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .required(true),
        )
}

fn build_delete_command() -> Command {
    Command::new("delete").about("Delete an existing shield")
}

fn build_list_command() -> Command {
    Command::new("list").about("List all shields")
}
