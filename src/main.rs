use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use dialoguer::Select;
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Language {
    Javascript,
    Typescript,
}

#[derive(Debug)]
enum ProofSystem {
    Groth16,
    Plonk,
}

#[derive(Debug)]
enum PackageManager {
    Yarn,
    NPM,
}

#[derive(Debug)]
struct SelectedArguments {
    project_path: String,
    language: Language,
    proof_system: ProofSystem,
    package_manager: PackageManager,
}

impl SelectedArguments {
    pub fn new() -> Self {
        SelectedArguments {
            project_path: String::new(),
            language: Language::Javascript,
            proof_system: ProofSystem::Groth16,
            package_manager: PackageManager::NPM,
        }
    }
}

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
        let mut args = SelectedArguments::new();

        args.project_path = path.to_string();
        get_language(&mut args);
        get_proof_system(&mut args);
        get_package_manager(&mut args);
        println!("generating project with following configs: \n{:#?}", args)
    }
}

fn get_selection(
    prompt: &str,
    options: &[&str],
    args: &mut SelectedArguments,
    setter: fn(&mut SelectedArguments, usize),
) {
    let selected = Select::new()
        .with_prompt(prompt)
        .items(options)
        .default(0)
        .interact()
        .expect("Failed to read input");

    setter(args, selected);
}

fn get_language(args: &mut SelectedArguments) {
    get_selection(
        "Please choose language",
        &["Javascript", "Typescript"],
        args,
        |args, selected| {
            args.language = match selected {
                0 => Language::Javascript,
                1 => Language::Typescript,
                _ => unreachable!(),
            };
        },
    );
}

fn get_proof_system(args: &mut SelectedArguments) {
    get_selection(
        "Please choose proof system",
        &["Groth16", "Plonk"],
        args,
        |args, selected| {
            args.proof_system = match selected {
                0 => ProofSystem::Groth16,
                1 => ProofSystem::Plonk,
                _ => unreachable!(),
            };
        },
    );
}

fn get_package_manager(args: &mut SelectedArguments) {
    get_selection(
        "Please choose package manager",
        &["NPM", "Yarn"],
        args,
        |args, selected| {
            args.package_manager = match selected {
                0 => PackageManager::NPM,
                1 => PackageManager::Yarn,
                _ => unreachable!(),
            };
        },
    );
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
