use crate::capture::interfaces::*;
use crate::structs::node::Node;

use regex::Regex;
use std::io::Error;
use std::thread;
use std::thread::JoinHandle;
use std::collections::BTreeMap;
use crate::capture::results::CaptureResult;
use std::fs::File;

pub fn init(ingress: String, egress: String, filter: Regex) -> Result<BTreeMap<String, Vec<CaptureResult>>, Error> {
    let node = Node::new(ingress, egress);
    let mut captures: BTreeMap<String, Vec<CaptureResult>> = BTreeMap::new();

    // Start a thread with a capture on the given ingress interface.
    let ingress_threads: Vec<JoinHandle<_>> = node
        .ingress()
        .iter()
        .map(|interface| {
            thread::spawn({
                let interface = interface.clone();
                move || {
                    captures.insert(interface.clone(), analyze_interface(true, interface)?);
                }
            })
        })
        .collect();

    // Start a thread with a capture on the given egress interface.
    let egress_threads: Vec<JoinHandle<_>> = node
        .egress()
        .iter()
        .map(|interface| {
            thread::spawn({
                let interface = interface.clone();
                move || {
                    captures.insert(interface.clone(), analyze_interface(false, interface)?);
                }
            })
        })
        .collect();

    // Serialize `captures` BTreeMap in order to load it later.
    let mut file = File::create("~/.monet/captures.mnt")?;
    bincode::serialize_into(&mut file, &captures)?;

    // Join threads as soon as they end their job.
    ingress_threads
        .into_iter()
        .map(|thread| thread.join().unwrap());
    egress_threads
        .into_iter()
        .map(|thread| thread.join().unwrap());

    Ok(captures)
}
