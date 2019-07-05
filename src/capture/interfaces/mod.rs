use crate::capture::results::*;
use crate::capture::{execute_once, get_interface_channels};
use crate::report;

use slog::{info, Logger};
use std::io::Error;

pub fn analyze_interface(ingress: bool, interface: String) -> Result<(), Error> {
    let reporter = report::init(ingress, interface.clone());

    let (mut tx, mut rx) = match get_interface_channels(&interface) {
        Ok(channels) => channels,
        Err(error) => {
            panic!(
                "Thread died while trying to capture in interface {}. Error is {}.",
                interface, error
            );
        }
    };

    loop {
        match execute_once((&mut tx, &mut rx)) {
            Ok(result) => {
                match result.0 {
                    CaptureResult::Other => continue,
                    _ => (),
                };
                info!(reporter, "{}", result.0);
            }
            Err(error) => match error {
                _ => (),
            },
        }
    }

    Ok(())
}
