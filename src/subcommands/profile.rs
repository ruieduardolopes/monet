use crate::analysis;
use crate::capture::interfaces::*;
use crate::capture::results::CaptureResult;
use crate::structs::node::Node;

use crossbeam_utils::thread;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Error;

pub fn init(
    ingress: String,
    egress: String,
    filter: Regex,
) -> Result<BTreeMap<String, Vec<CaptureResult>>, Error> {
    let node = Node::new(ingress, egress);
    let mut captures: BTreeMap<String, Vec<CaptureResult>> = BTreeMap::new();

    // Start a thread with a capture on the given ingress interface.
    for interface_index in 0..node.ingress().len() {
        thread::scope(|scope| {
            let filter = filter.clone();
            scope.spawn(|_| {
                let interface = node.ingress().get(interface_index).unwrap().clone();
                captures.insert(
                    format!("{}-{}", "ingress", interface),
                    run_capture(true, interface, filter).unwrap(),
                )
            });
        });
    }

    // Start a thread with a capture on the given egress interface.
    for interface_index in 0..node.egress().len() {
        thread::scope(|scope| {
            let filter = filter.clone();
            scope.spawn(|_| {
                let interface = node.egress().get(interface_index).unwrap().clone();
                captures.insert(
                    format!("{}-{}", "egress", interface),
                    run_capture(true, interface, filter).unwrap(),
                )
            });
        });
    }

    // Analyze differences, by protocol (according to a filter), between what passes as in- and egress.
    analysis::run_internal_struct(&captures);

    // Serialize `captures` BTreeMap in order to load it later.
    let mut file = File::create("~/.monet/captures.mnt").unwrap();
    bincode::serialize_into(&mut file, &captures).unwrap();

    Ok(captures)
}
