use crate::capture::results::*;
use crate::capture::{execute_once, get_interface_channels};
use std::io::Error;

pub fn analyze_interface(interface: String) -> Result<(), Error> {
    let (mut tx, mut rx) = match get_interface_channels(&interface) {
        Ok(channels) => channels,
        Err(error) => {
            panic!(
                "Thread's dead while trying to capture in interface {}. Error is {}.",
                interface, error
            );
        }
    };

    loop {
        match execute_once((&mut tx, &mut rx)) {
            Ok(result) => match result.0 {
                CaptureResult::Frame(_) => {}
                CaptureResult::Fragment(_) => {}
                CaptureResult::LastFragment(_) => {}
                CaptureResult::SequenceParameterSet => {}
                CaptureResult::PictureParameterSet => {}
                CaptureResult::Stream(_, _, _, _) => {}
                CaptureResult::Other => {}
            },
            Err(error) => match error {
                _ => (),
            },
        }
    }

    Ok(())
}
