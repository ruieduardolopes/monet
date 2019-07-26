use crate::capture::results::*;
use crate::capture::{execute_once, get_interface_channels};
use crate::{capture, report};

use crossbeam_deque::Worker;
use crossbeam::channel::Sender;
use regex::Regex;
use slog::{info, Logger};
use std::io::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use crossbeam::channel::Receiver;

pub fn run_capture(
    ingress: bool,
    interface: &String,
    filter: &Regex,
    session: &Arc<AtomicUsize>,
    to_main_thread: Sender<Vec<CaptureResult>>,
) -> Result<() /*Vec<CaptureResult>*/, Error> {
    let reporter = report::init(ingress, interface.clone());
    let mut internal_report: Vec<CaptureResult> = Vec::new();

    // Spawn capture thread with filter thread within.
    let capture_thread = thread::spawn({
        let worker_on_queue: Worker<CaptureResult> = Worker::new_fifo();

        // Spawn filter thread with a stealer to the fifo queue variable.
        let stealer = worker_on_queue.stealer();
        /*internal_report =*/ thread::spawn({
            let filter = filter.clone();
            let stealer = stealer.clone();
            let session = session.clone();
            let to_main_thread = to_main_thread.clone();
            let mut internal_report = internal_report.clone();
            move || {
                capture::filter::init(&session, &filter, stealer, &mut internal_report);
                println!(" capt is {:?}", internal_report);
                to_main_thread.send(internal_report.clone());
                report::report(&reporter, &internal_report);
//                internal_report
            }
        });
//        .join()
//        .unwrap();

        // Create interface channels in order to process capture in it.
        let (mut tx, mut rx) = match get_interface_channels(&interface) {
            Ok(channels) => channels,
            Err(error) => {
                panic!(
                    "Thread died while trying to capture in interface {}. Error is {}.",
                    interface, error
                );
            }
        };

        // Perform capture in given channels.
        move || loop {
//            to_main_thread.try_send(true);
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
    });

    Ok(())
}
