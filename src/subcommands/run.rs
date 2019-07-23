use crossbeam_utils::thread;
use regex::Regex;
use std::collections::BTreeMap;
use std::io::Error;

use crate::capture::interfaces::*;
use crate::capture::results::CaptureResult;
use crate::structs::node::Node;
use std::fs::File;

fn start_captures(
    captures: &mut BTreeMap<String, Vec<CaptureResult>>,
    interfaces: &Vec<String>,
    is_ingress: bool,
    filter: &Regex,
) {
    for index in 0..interfaces.len() {
        thread::scope(|scope| {
            scope.spawn(|_| {
                let interface = interfaces.get(index).unwrap().clone();
                captures.insert(
                    format!(
                        "{}-{}",
                        if is_ingress { "ingress" } else { "egress" },
                        interface
                    ),
                    run_capture(true, interface, filter).unwrap(),
                )
            });
        });
    }
}

pub fn init(ingress: String, egress: String, filter: Regex) -> Result<(), Error> {
    let mut captures: BTreeMap<String, Vec<CaptureResult>> = BTreeMap::new();

    // Set this execution as a node with ingress and egress interfaces.
    let node = Node::new(ingress, egress);

    // Start both ingress and egress captures.
    start_captures(&mut captures, &node.ingress(), true, &filter);
    start_captures(&mut captures, &node.egress(), false, &filter);

    // Serialize captures BTreeMap in file to load it later in analysis.
    let mut file = File::create("~/.monet/captures.monet").unwrap();
    bincode::serialize_into(&mut file, &captures).unwrap();

    Ok(())
}
