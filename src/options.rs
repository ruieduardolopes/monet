use clap::{App, Arg, ArgMatches, SubCommand};

pub fn get_options_from_cli() -> ArgMatches<'static> {
    App::new("Monet: Monitor a Network")
        .version("0.1.0")
        .author("André Nascimento and Rui Lopes")
        .about("Tool to get some insights on a network or interfaces")
        .subcommand(
            SubCommand::with_name("run")
                .arg(
                    Arg::with_name("ingress-interfaces")
                        .takes_value(true)
                        .long("ingress-interfaces")
                        .short("i")
                        .required(true)
                        .help("Set of ingress interfaces, between quotes: i.e. \"enp0s1,wlan0\"")
                )
                .arg(
                    Arg::with_name("egress-interfaces")
                        .takes_value(true)
                        .long("egress-interfaces")
                        .short("e")
                        .required(true)
                        .help("Set of egress interfaces, between quotes: i.e. \"enp0s1,wlan0\"")
                )
                .arg(
                    Arg::with_name("filter")
                        .takes_value(true)
                        .long("filter")
                        .short("f")
                        .required(false)
                        .help("Filter to apply to both ingress and egress packets, as regex: i.e. [icmp|h264]")
                )
                .about("Capture traffic passing through a node, passing from an ingress interface to an egress one.")
        )
        .subcommand(
            SubCommand::with_name("analyze")
                .about("Analyze already captured traffic, whose data is inside ~/.monet directory.")
        )
        .subcommand(
            SubCommand::with_name("profile")
                .arg(
                    Arg::with_name("ingress-interfaces")
                        .takes_value(true)
                        .long("ingress-interfaces")
                        .short("i")
                        .required(true)
                        .help("Set of ingress interfaces, between quotes: i.e. \"enp0s1,wlan0\"")
                )
                .arg(
                    Arg::with_name("egress-interfaces")
                        .takes_value(true)
                        .long("egress-interfaces")
                        .short("e")
                        .required(true)
                        .help("Set of egress interfaces, between quotes: i.e. \"enp0s1,wlan0\"")
                )
                .arg(
                    Arg::with_name("filter")
                        .takes_value(true)
                        .long("filter")
                        .short("f")
                        .required(false)
                        .help("Filter to apply to both ingress and egress packets, as regex: i.e. [icmp|h264]")
                )
                .about("Capture and analyze traffic passing through a node, passing from an ingress interface to an egress one.")
        )
        .get_matches()
}
