use clap::{App, Arg, ArgMatches, SubCommand};

pub fn get_options_from_cli() -> ArgMatches<'static> {
    App::new("Monet: Monitor a Network")
        .version("0.1.0")
        .author("Rui Lopes")
        .about("Tool to get some insights on a network or interfaces")
        .subcommand(
            SubCommand::with_name("perf")
                .about("Run a network performance test.")
                .subcommand(SubCommand::with_name("server").about("Run an instance as a server."))
                .subcommand(SubCommand::with_name("client").about("Run an instance as a client.")),
        )
        .subcommand(SubCommand::with_name("interface").about("Analyze"))
        .get_matches()
}
