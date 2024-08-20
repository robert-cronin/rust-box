use clap::{App, Arg, SubCommand};
use std::process::exit;
mod container;

fn main() {
    let matches = App::new("RustBox")
        .version("0.1.0")
        .author("Robert Cronin")
        .about("A rusty Docker clone")
        .subcommand(
            SubCommand::with_name("run").about("Run a container").arg(
                Arg::with_name("command")
                    .help("The command to run")
                    .required(true)
                    .multiple(true),
            ),
        )
        .get_matches();

    if !nix::unistd::Uid::effective().is_root() {
        eprintln!("Error: This program needs to be run with root privileges. Please use sudo.");
        exit(1);
    }

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let command: Vec<String> = run_matches
                .values_of("command")
                .unwrap()
                .map(String::from)
                .collect();
            println!("Attempting to run command: {:?}", command);
            match container::run(command) {
                Ok(_) => println!("Container exited successfully"),
                Err(e) => {
                    eprintln!("Error running container: {:?}", e);
                    exit(1);
                }
            }
        }
        _ => {
            println!("Invalid command. Use --help for usage information.");
            exit(1);
        }
    }
}
