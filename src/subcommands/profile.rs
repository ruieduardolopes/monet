use crate::capture::interfaces::*;

use crate::structs::node::Node;
use regex::Regex;
use std::io::Error;
use std::thread;
use std::thread::JoinHandle;

pub fn init(ingress: String, egress: String, filter: Regex) -> Result<(), Error> {
    let node = Node::new(ingress, egress);

    // Start a thread with a capture on the given ingress interface.
    let ingress_threads: Vec<JoinHandle<_>> = node
        .ingress()
        .iter()
        .map(|interface| {
            thread::spawn({
                let interface = interface.clone();
                move || {
                    analyze_interface(interface);
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
                    analyze_interface(interface);
                }
            })
        })
        .collect();

    // Join threads as soon as they end their job.
    ingress_threads
        .into_iter()
        .map(|thread| thread.join().unwrap());
    egress_threads
        .into_iter()
        .map(|thread| thread.join().unwrap());

    Ok(())
}
