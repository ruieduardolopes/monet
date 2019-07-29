use crate::capture::results::*;
use crate::capture::{execute_once, get_interface_channels};
use crate::{capture, report};

use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use crossbeam_deque::Worker;
use regex::Regex;
use slog::{info, Logger};
use std::io::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{thread, fs};
use std::time::Duration;

#[cfg(not(target_os = "macos"))]
fn get_specs(interface: String, reporter: Logger, session: &Arc<AtomicUsize>) {
    const SIGINT: usize = signal_hook::SIGINT as usize;

    let mut collisions: i64;
    let mut multicast: i64;
    let mut rx_bytes: i64;
    let mut rx_compressed: i64;
    let mut rx_crc_errors: i64;
    let mut rx_dropped: i64;
    let mut rx_errors: i64;
    let mut rx_fifo_errors: i64;
    let mut rx_frame_errors: i64;
    let mut rx_length_errors: i64;
    let mut rx_missed_errors: i64;
    let mut rx_nohandler: i64;
    let mut rx_over_errors: i64;
    let mut rx_packets: i64;
    let mut tx_aborted_errors: i64;
    let mut tx_bytes: i64;
    let mut tx_carrier_errors: i64;
    let mut tx_compressed: i64;
    let mut tx_dropped: i64;
    let mut tx_errors: i64;
    let mut tx_fifo_errors: i64;
    let mut tx_heartbeat_errors: i64;
    let mut tx_packets: i64;
    let mut tx_window_errors: i64;
    let mut timestamp;

    loop {
        match session.load(Ordering::Relaxed) {
            SIGINT => break,
            _ => {}
        }

        collisions = fs::read_to_string(format!("/sys/class/net/{}/statistics/collisions", interface)).unwrap().parse::<i64>().unwrap();
        multicast = fs::read_to_string(format!("/sys/class/net/{}/statistics/multicast", interface)).unwrap().parse::<i64>().unwrap();
        rx_bytes = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_bytes", interface)).unwrap().parse::<i64>().unwrap();
        rx_compressed = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_compressed", interface)).unwrap().parse::<i64>().unwrap();
        rx_crc_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_crc_errors", interface)).unwrap().parse::<i64>().unwrap();
        rx_dropped = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_dropped", interface)).unwrap().parse::<i64>().unwrap();
        rx_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_errors", interface)).unwrap().parse::<i64>().unwrap();
        rx_fifo_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_fifo_errors", interface)).unwrap().parse::<i64>().unwrap();
        rx_frame_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_frame_errors", interface)).unwrap().parse::<i64>().unwrap();
        rx_length_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_length_errors", interface)).unwrap().parse::<i64>().unwrap();
        rx_missed_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_missed_errors", interface)).unwrap().parse::<i64>().unwrap();
        rx_nohandler = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_nohandler", interface)).unwrap().parse::<i64>().unwrap();
        rx_over_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_over_errors", interface)).unwrap().parse::<i64>().unwrap();
        rx_packets = fs::read_to_string(format!("/sys/class/net/{}/statistics/rx_packets", interface)).unwrap().parse::<i64>().unwrap();
        tx_aborted_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_aborted_errors", interface)).unwrap().parse::<i64>().unwrap();
        tx_bytes = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_bytes", interface)).unwrap().parse::<i64>().unwrap();
        tx_carrier_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_carrier_errors", interface)).unwrap().parse::<i64>().unwrap();
        tx_compressed = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_compressed", interface)).unwrap().parse::<i64>().unwrap();
        tx_dropped = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_dropped", interface)).unwrap().parse::<i64>().unwrap();
        tx_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_errors", interface)).unwrap().parse::<i64>().unwrap();
        tx_fifo_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_fifo_errors", interface)).unwrap().parse::<i64>().unwrap();
        tx_heartbeat_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_heartbeat_errors", interface)).unwrap().parse::<i64>().unwrap();
        tx_packets = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_packets", interface)).unwrap().parse::<i64>().unwrap();
        tx_window_errors = fs::read_to_string(format!("/sys/class/net/{}/statistics/tx_window_errors", interface)).unwrap().parse::<i64>().unwrap();
        timestamp = time::now_utc().to_timespec().sec;

        info!(
            reporter,
            "[monet] {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            collisions,
            multicast,
            rx_bytes,
            rx_compressed,
            rx_crc_errors,
            rx_dropped,
            rx_errors,
            rx_fifo_errors,
            rx_frame_errors,
            rx_length_errors,
            rx_missed_errors,
            rx_nohandler,
            rx_over_errors,
            rx_packets,
            tx_aborted_errors,
            tx_bytes,
            tx_carrier_errors,
            tx_compressed,
            tx_dropped,
            tx_errors,
            tx_fifo_errors,
            tx_heartbeat_errors,
            tx_packets,
            tx_window_errors,
            timestamp
        );

        thread::sleep(Duration::from_secs(1));
    }
}

pub fn run_capture(
    ingress: bool,
    interface: &String,
    filter: &Regex,
    session: &Arc<AtomicUsize>,
    to_main_thread: Sender<Vec<CaptureResult>>,
) -> Result<(), Error> {
    let reporter = report::init(ingress, interface.clone());
    let interface_reporter =
        report::init(ingress, String::from(format!("spec-{}", interface.clone())));
    let mut internal_report: Vec<CaptureResult> = Vec::new();

    // Spawn capture thread with filter thread within.
    let capture_thread = thread::spawn({
        let worker_on_queue: Worker<CaptureResult> = Worker::new_fifo();

        // Spawn filter thread with a stealer to the fifo queue variable.
        let stealer = worker_on_queue.stealer();
        thread::spawn({
            let filter = filter.clone();
            let stealer = stealer.clone();
            let session = session.clone();
            let to_main_thread = to_main_thread.clone();
            let mut internal_report = internal_report.clone();
            move || {
                capture::filter::init(&session, &filter, stealer, &mut internal_report);
                to_main_thread.send(internal_report.clone());
                report::report(&reporter, &internal_report);
            }
        });

        // Spawn interface spec reporter thread.
        #[cfg(not(target_os = "macos"))]
        thread::spawn({
            let interface = interface.clone();
            let session = session.clone();
            let reporter = interface_reporter.clone();
            move || {
                report_specs(interface, reporter, session);
            }
        });

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
