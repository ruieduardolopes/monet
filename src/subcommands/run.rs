use crossbeam::channel::*;
use crossbeam_utils::thread;
use crossbeam_utils::thread::*;
use rayon::prelude::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{env, process};

use crate::capture::interfaces::*;
use crate::capture::results::CaptureResult;
use crate::structs::node::status::NodeStatus;
use crate::structs::node::Node;

pub fn init(ingress: String, egress: String, filter: Regex, export: bool) -> Result<(), Error> {
    // Start both ingress and egress captures.
    thread::scope(|scope| {
        let home = match env::var_os("HOME") {
            None => {
                panic!("$HOME not set.");
            }
            Some(path) => PathBuf::from(path),
        };
        std::fs::remove_dir_all(format!("{}/.monet/", home.display())).unwrap();
        std::fs::create_dir(format!("{}/.monet/", home.display())).unwrap();

        // Override instructions for handling Ctrl-C signal.
        let session = Arc::new(AtomicUsize::new(0));
        const SIGINT: usize = signal_hook::SIGINT as usize;
        signal_hook::flag::register_usize(signal_hook::SIGINT, Arc::clone(&session), SIGINT);

        let (send_signal_thread, recv_signal_thread) = crossbeam::channel::unbounded();
        let (send_main_thread, recv_main_thread) = crossbeam::channel::unbounded();

        let signal_handle = scope.spawn({
            let session = session.clone();
            let send_signal_thread = send_signal_thread.clone();
            move |_| {
                loop {
                    match session.load(Ordering::Relaxed) {
                        SIGINT => {
                            send_signal_thread.send(true); // Send signal through channel to capturing selection...
                        }
                        _ => {}
                    }
                }
            }
        });

        // Set this execution as a node with ingress and egress interfaces.
        let node = Node::new(ingress, egress);
        match node.status() {
            NodeStatus::NotANode => {
                panic!("Quitting, since no interfaces were given as both ingress and egress.")
            }
            _ => println!("Running monet on a {}...", node.status()),
        }

        let mut counter: usize = 0;

        // Start captures for ingress interfaces.
        node.all().par_iter().for_each(|(interface, is_ingress)| {
            let mut capture: Vec<CaptureResult> = Vec::new();
            let capture_handle = scope.spawn({
                let interface = interface.clone();
                let filter = filter.clone();
                let session = session.clone();
                let send_main_thread = send_main_thread.clone();
                move |_| run_capture(true, &interface, &filter, &session, send_main_thread).unwrap()
            });
            loop {
                select! {
                    recv(recv_signal_thread) -> kill => {
                        break;
                    },
                    recv(recv_main_thread) -> result => {
                    }
                    default => {
                        capture = recv_main_thread.recv().unwrap();
                    }
                }
            }
            if export {
                let mut file = OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(if *is_ingress {
                        println!(
                            "\nTrying to create {}",
                            format!("{}/.monet/ingress-{}.monet", home.display(), interface)
                        );
                        format!("{}/.monet/ingress-{}.monet", home.display(), interface)
                    } else {
                        println!(
                            "\nTrying to create {}",
                            format!("{}/.monet/egress-{}.monet", home.display(), interface)
                        );
                        format!("{}/.monet/egress-{}.monet", home.display(), interface)
                    })
                    .unwrap();
                bincode::serialize_into(&mut file, &capture).unwrap();
            }
        });
        std::process::exit(0);
    });

    Ok(())
}
