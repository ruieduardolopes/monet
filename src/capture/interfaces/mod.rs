use std::io::Error;
use crate::capture::{execute_once, get_interface_channels};

pub fn analyze_interface(interface: String) -> Result<(), Error> {
    let (mut tx, mut rx) = match get_interface_channels(&interface) {
        Ok(channels) => {channels},
        Err(error) => {
            panic!("Thread's dead while trying to capture in interface {}. Error is {}.", interface, error);
        },
    };

    loop {
        match execute_once((&mut tx, &mut rx)) {
            Ok(result) => {},
            Err(error) => {
                match error {
                    _ => ()
                }
            },
        }
    }

    Ok(())
}