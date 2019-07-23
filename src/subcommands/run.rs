use crossbeam_utils::thread;
use crossbeam_utils::thread::*;
use rayon::prelude::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::{env, process};
use std::path::PathBuf;

use crate::capture::interfaces::*;
use crate::capture::results::CaptureResult;
use crate::structs::node::Node;

//fn start_captures(
//    scope: &Scope,
//    session: &Arc<AtomicUsize>,
//    captures: &'static mut BTreeMap<String, Vec<CaptureResult>>,
//    interfaces: &'static Vec<String>,
//    is_ingress: bool,
//    filter: &'static Regex,
//) {
//    interfaces.into_iter().for_each(|interface| {
//        captures.insert(
//            format!(
//                "{}-{}",
//                if is_ingress { "ingress" } else { "egress" },
//                interface
//            ),
//            run_capture(true, interface, filter).unwrap(),
//        );
//    });
//}

pub fn init(ingress: String, egress: String, filter: Regex) -> Result<(), Error> {
    println!("Initializing");

    // Start both ingress and egress captures.
    thread::scope(|scope| {
        // Override instructions for handling Ctrl-C signal.
        let session = Arc::new(AtomicUsize::new(0));
        const SIGINT: usize = signal_hook::SIGINT as usize;
        signal_hook::flag::register_usize(signal_hook::SIGINT, Arc::clone(&session), SIGINT);

        let home = match env::var_os("HOME") {
            None => { println!("$HOME not set."); process::exit(1) }
            Some(path) => PathBuf::from(path),
        };

        // Set this execution as a node with ingress and egress interfaces.
        let node = Node::new(ingress, egress);

        // Start captures for ingress interfaces.
        node.all().par_iter().for_each(|(interface, is_ingress)| {
            let capture = scope
                .spawn({
                    println!("Hello");
                    let interface = interface.clone();
                    let filter = filter.clone();
                    let session = session.clone();
                    move |_| run_capture(true, &interface, &filter, &session).unwrap()
                })
                .join()
                .unwrap();
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(if *is_ingress {
                    println!("Trying to create {}", format!("{}/.monet/ingress-{}.monet", home.display(), interface)); format!("{}/.monet/ingress-{}.monet", home.display(), interface)
                } else {
                    println!("Trying to create {}", format!("{}/.monet/egress-{}.monet", home.display(), interface)); format!("{}/.monet/egress-{}.monet", home.display(), interface)
                }).unwrap();
            bincode::serialize_into(&mut file, &capture).unwrap();
        });
    });

    Ok(())
}
