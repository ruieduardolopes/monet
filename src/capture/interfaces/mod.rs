use crate::capture::results::*;
use crate::capture::{execute_once, get_interface_channels};
use crate::{report, capture};

use slog::{info, Logger};
use std::io::Error;
use regex::Regex;
use crossbeam_deque::Worker;
use std::thread;

pub fn run_capture
(ingress: bool, interface: String, filter: Regex) -> Result<Vec<CaptureResult>, Error> {
    let reporter = report::init(ingress, interface.clone());
    let mut internal_report: Vec<CaptureResult> = Vec::new();
    let worker_on_queue: Worker<CaptureResult> = Worker::new_fifo();

    let (mut tx, mut rx) = match get_interface_channels(&interface) {
        Ok(channels) => channels,
        Err(error) => {
            panic!(
                "Thread died while trying to capture in interface {}. Error is {}.",
                interface, error
            );
        }
    };

    // Spawn filter thread with a stealer to the fifo queue variable.
//    let stealer = worker_on_queue.stealer();
//    let filter_thread = thread::spawn({
//        let filter = filter.clone();
//        let stealer = stealer.clone();
//        let mut internal_report = internal_report.clone();
//        move || {
//            capture::filter::init(&filter, stealer, &mut internal_report);
//        }
//    });

    loop {
        match execute_once((&mut tx, &mut rx)) {
            Ok(result) => {
                match result.0 {
                    CaptureResult::Other(timestamp) => continue,
                    _ => (),
                };
                worker_on_queue.push(result.0);
            }
            Err(error) => match error {
                _ => (),
            },
        }
    }

    filter_thread.join().expect("Cannot stop filter thread from its work... Panic!");

//    filter_thread
//        .join()
//        .expect("Cannot stop filter thread from its work... Panic!");

    Ok(internal_report)
}
