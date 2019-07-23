#![allow(unused_assignments)]
#![allow(unused_imports)]

pub mod analysis;
pub mod capture;
pub mod options;
pub mod report;
pub mod structs;
pub mod subcommands;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

use regex::Regex;
use rayon::prelude::*;

fn main() {
    let cliopts = options::get_options_from_cli();

    match cliopts.subcommand_name() {
        Some(subcommand_string) => match subcommand_string {
            "run" => {
                let mut ingress = String::new();
                let mut egress = String::new();
                let mut filter;
                match cliopts
                    .subcommand_matches(subcommand_string)
                    .unwrap()
                    .value_of("ingress-interfaces")
                {
                    Some(ingress_interfaces) => {
                        ingress = String::from(ingress_interfaces);
                    }
                    None => {
                        panic!("Value of ingress interface was not assigned.");
                    }
                }
                match cliopts
                    .subcommand_matches(subcommand_string)
                    .unwrap()
                    .value_of("egress-interfaces")
                {
                    Some(egress_interfaces) => {
                        egress = String::from(egress_interfaces);
                    }
                    None => {
                        panic!("Value of egress interface was not assigned.");
                    }
                }
                match cliopts
                    .subcommand_matches(subcommand_string)
                    .unwrap()
                    .value_of("filter")
                {
                    Some(filter_string) => filter = Regex::new(filter_string).unwrap(),
                    None => filter = Regex::new(".*").unwrap(),
                }
                match subcommands::run::init(ingress, egress, filter) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            "analyze" => match subcommands::analyze::init() {
                Ok(_) => {}
                Err(_) => {}
            },
            "profile" => {
                let mut ingress = String::new();
                let mut egress = String::new();
                let mut filter;
                match cliopts
                    .subcommand_matches(subcommand_string)
                    .unwrap()
                    .value_of("ingress-interfaces")
                {
                    Some(ingress_interfaces) => {
                        ingress = String::from(ingress_interfaces);
                    }
                    None => {
                        panic!("Value of ingress interface was not assigned.");
                    }
                }
                match cliopts
                    .subcommand_matches(subcommand_string)
                    .unwrap()
                    .value_of("egress-interfaces")
                {
                    Some(egress_interfaces) => {
                        egress = String::from(egress_interfaces);
                    }
                    None => {
                        panic!("Value of egress interface was not assigned.");
                    }
                }
                match cliopts
                    .subcommand_matches(subcommand_string)
                    .unwrap()
                    .value_of("filter")
                {
                    Some(filter_string) => filter = Regex::new(filter_string).unwrap(),
                    None => filter = Regex::new(".*").unwrap(),
                }
                match subcommands::profile::init(ingress, egress, filter) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            _ => panic!(
                "The inserted subcommand {} does not belong to the list of subcommands.",
                subcommand_string
            ),
        },
        None => panic!("No subcommand was inserted. This execution needs one to proceed."),
    }
}
