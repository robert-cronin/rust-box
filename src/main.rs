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
                    .takes_value(true),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let command = run_matches.value_of("command").unwrap();
            match container::run(command) {
                Ok(_) => println!("Container exited successfully"),
                Err(e) => {
                    eprintln!("Error running container: {}", e);
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
