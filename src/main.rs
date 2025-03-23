use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use dialoguer::Select;
use std::path::Path;
use std::{fs};

fn main() {
    let matches = Command::new("shield")
        .version("0.1.0")
        .about("Shield CLI tool is a development framework for circom developers")
        .subcommand(
            Command::new("init")
                .about("generates the boilerplate code")
                .arg(
                    Arg::new("path")
                        .value_parser(value_parser!(String))
                        .action(ArgAction::Set)
                        .required(true),
                ),
        )
        .subcommand(Command::new("delete").about("Delete an existing shield"))
        .subcommand(Command::new("list").about("List all shields"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", matched_args)) => {
            init(matched_args);
        }
        Some(("delete", _)) => {
            println!("Deleting a shield...");
        }
        Some(("list", _)) => {
            println!("Listing all shields...");
        }
        _ => {
            println!("Use --help to see available commands.");
        }
    }
}

fn init(matched_args: &ArgMatches) {
    if let Some(path) = matched_args.get_one::<String>("path") {
        let options = vec![
            "Create a new shield",
            "Overwrite the existing shield",
            "Cancel",
        ];

        let selection = Select::new()
            .with_prompt("Please choose an option")
            .items(&options)
            .default(0)
            .interact()
            .expect("Failed to read input");
        match selection {
            0 => {
                println!("Creating a new shield...");
                generate(path);
            }
            1 => {
                println!("Overwriting the existing shield...");
                generate(path);
            }
            2 => {
                println!("Operation canceled.");
                return;
            }
            _ => unreachable!(),
        }
    }
}

fn generate(path: &str) {
    println!("Creating a new shield at: {}", path);
    let file_path = Path::new(path);
    if file_path.exists() {
        println!("The shield already exists at: {}", path);
    } else {
        fs::create_dir_all(file_path).expect("Failed to create directory");
        println!("Shield created at: {}", path);
    }
}
