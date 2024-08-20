use clap::{App, Arg, SubCommand};
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

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let command: Vec<&str> = run_matches.values_of("command").unwrap().collect();
            match container::run(command) {
                Ok(_) => println!("Container exited successfully"),
                Err(e) => eprintln!("Error running container: {}", e),
            }
        }
        _ => println!("Invalid command. Use --help for usage information."),
    }
}
