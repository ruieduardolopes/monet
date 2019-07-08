use std::collections::BTreeMap;

use crate::capture::results::*;
use std::fs::File;
use std::io::Error;

pub fn run_struct(captures: &BTreeMap<String, Vec<CaptureResult>>) -> Result<(), Error> {
    // Retrieve vector with all keys from captures.
    let mut captures_keys = captures.keys().cloned().collect::<Vec<String>>();

    // Create BTreeMap only with ingress interface captures.
    let mut ingress_captures_keys = captures_keys.clone();
    ingress_captures_keys.retain(|key| key.contains("ingress"));
    let mut ingress_captures: BTreeMap<String, &Vec<CaptureResult>> = BTreeMap::new();
    for item in ingress_captures_keys {
        ingress_captures.insert(item.clone(), captures.get(&item).unwrap().clone());
    }

    // Create BTreeMap only with egress interface captures.
    let mut egress_captures_keys = captures_keys.clone();
    egress_captures_keys.retain(|key| key.contains("egress"));
    let mut egress_captures: BTreeMap<String, &Vec<CaptureResult>> = BTreeMap::new();
    for item in egress_captures_keys {
        egress_captures.insert(item.clone(), captures.get(&item).unwrap().clone());
    }



    Ok(())
}

pub fn run_binary_file(serialized_file: File) -> Result<(), Error> {
    // Deserialize captures BTreeMap.
    let captures: BTreeMap<String, Vec<CaptureResult>> =
        bincode::deserialize_from(&serialized_file).unwrap();

    Ok(())
}
