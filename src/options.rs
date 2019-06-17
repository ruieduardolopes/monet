use clap::{App, SubCommand, Arg, ArgMatches};

pub fn get_options_from_cli() -> ArgMatches<'static> {
    App::new("Monet: Monitor a Network")
        .version("0.1.0")
        .author("Rui Lopes")
        .about("Tool to get some insights on a network")
        .subcommand(
            SubCommand::with_name("server")
                .about("Run an instance as a server.")
//                .arg(
//                    Arg::with_name("input")
//                        .short("i")
//                        .long("input")
//                        .value_name("input-file")
//                        .help("Path to input file (an H.264 MPEG video)")
//                        .takes_value(true)
//                        .required(true),
//                ),
        ).subcommand(
            SubCommand::with_name("client")
                .about("Run an instance as a client.")
//                .arg(
//                    Arg::with_name("input")
//                        .short("i")
//                        .long("input")
//                        .value_name("input-file")
//                        .help("Path to input file (an H.264 MPEG video)")
//                        .takes_value(true)
//                        .required(true),
//        )
        )
        .get_matches()
}
