use crate::capture::results::CaptureResult;
use crossbeam_deque::{Steal, Stealer};
use regex::Regex;
use std::io::Error;

pub fn init(
    filter: &Regex,
    stealer: Stealer<CaptureResult>,
    internal_reporter: &mut Vec<CaptureResult>,
) -> Result<(), Error> {
    loop {
        match stealer.steal() {
            Steal::Success(result) => {
                // TODO Apply filtering criteria here...
                internal_reporter.push(result);
                println!("internal is {:?}", internal_reporter);
            }
            _ => {

            }
        }
    }
}
