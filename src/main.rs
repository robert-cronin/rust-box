use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("RustBox")
        .version("0.1.0")
        .author("Robert Cronin")
        .about("A rusty Docker clone")
        .subcommand(SubCommand::with_name("run")
            .about("Run a container")
            .arg(Arg::with_name("image")
                .help("The image to run")
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let image = run_matches.value_of("image").unwrap();
            println!("Running container with image: {}", image);
            // TODO: Implement container creation and execution
        }
        _ => println!("Invalid command. Use --help for usage information."),
    }
}
