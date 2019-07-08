use std::collections::BTreeMap;

use crate::capture::results::*;
use std::io::Error;
use std::fs::File;

pub fn run_struct(captures: BTreeMap<String, Vec<CaptureResult>>) -> Result<(), Error> {
    Ok(())

}

pub fn run_binary_file(serialized_file: File) -> Result<(), Error> {
    // Deserialize captures BTreeMap.
    let captures: BTreeMap<String, Vec<CaptureResult>> = bincode::deserialize_from(&serialized_file).unwrap();

    Ok(())
}