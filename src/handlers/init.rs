use super::Result;
use crate::{
    core::error::ShieldError,
    utils::{
        custom_enum::{Language, PackageManager, ProofSystem},
        custom_struct::SelectedArguments, fs::generate,
    },
};
use clap::ArgMatches;
use dialoguer::Select;

pub fn handle(args: &ArgMatches) -> Result<()> {
    let path = args
        .get_one::<String>("path")
        .ok_or(ShieldError::MissingArgument("path".to_string()))?;

    let mut args = SelectedArguments::new();

    args.set_project_path(path.to_string());
    get_language(&mut args);
    get_proof_system(&mut args);
    get_package_manager(&mut args);
    println!("generating project with following configs: \n{:#?}", args);
    generate(path);

    Ok(())
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
            let lang = match selected {
                0 => Language::Javascript,
                1 => Language::Typescript,
                _ => unreachable!(),
            };
            args.set_language(lang)
        },
    );
}

fn get_proof_system(args: &mut SelectedArguments) {
    get_selection(
        "Please choose proof system",
        &["Groth16", "Plonk"],
        args,
        |args, selected| {
            let proof_system = match selected {
                0 => ProofSystem::Groth16,
                1 => ProofSystem::Plonk,
                _ => unreachable!(),
            };
            args.set_proof_system(proof_system);
        },
    );
}

fn get_package_manager(args: &mut SelectedArguments) {
    get_selection(
        "Please choose package manager",
        &["NPM", "Yarn"],
        args,
        |args, selected| {
            let package_manager = match selected {
                0 => PackageManager::NPM,
                1 => PackageManager::Yarn,
                _ => unreachable!(),
            };
            args.set_package_manager(package_manager);
        },
    );
}
