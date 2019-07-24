pub mod filters;

use crate::capture::results::CaptureResult;
use crate::capture::filter::filters::FilterValues;

use crossbeam_deque::{Steal, Stealer};
use regex::Regex;
use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn init(
    session: &Arc<AtomicUsize>,
    filter: &Regex,
    stealer: Stealer<CaptureResult>,
    internal_reporter: &mut Vec<CaptureResult>,
) -> Result<(), Error> {
    const SIGINT: usize = signal_hook::SIGINT as usize;

    loop {
        match session.load(Ordering::Relaxed) {
            SIGINT => {
                break
            },
            _ => {}
        }

        match stealer.steal() {
            Steal::Success(result) => {
                if filter.is_match(result.clone().into()) {
                    internal_reporter.push(result);
                }
            }
            _ => {

            }
        }
    }

    Ok(())
}
