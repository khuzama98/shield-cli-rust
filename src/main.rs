mod cli;
mod core;
mod handlers;
mod utils;

use handlers::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let matches = cli::commands::build_cli().get_matches();

    match matches.subcommand() {
        Some(("init", args)) => handlers::init::handle(args),
        // Some(("delete", args)) => handlers::delete::handle(args),
        // Some(("list", args)) => handlers::list::handle(args),
        _ => Ok(println!("Use --help to see available commands.")),
    }
}
