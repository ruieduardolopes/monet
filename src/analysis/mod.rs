use crate::capture::results::*;

use ipnetwork::IpNetwork;
use pnet::datalink::NetworkInterface;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Error;

fn is_this_machine(address: IpNetwork) -> bool {
    let all_interfaces: Vec<NetworkInterface> = pnet::datalink::interfaces();

    match address {
        IpNetwork::V4(address) => all_interfaces
            .iter()
            .find(|interface| interface.ips.contains(&IpNetwork::V4(address)))
            .is_some(),
        IpNetwork::V6(address) => all_interfaces
            .iter()
            .find(|interface| interface.ips.contains(&IpNetwork::V6(address)))
            .is_some(),
    }
}

pub fn run_internal_struct(captures: &BTreeMap<String, Vec<CaptureResult>>) -> Result<(), Error> {
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

    // Discard everything on ingress interfaces coming to this machine (and count no. of packets).


    // Discard everything on egress interfaces going out of this machine (and count no. of packets).


    // Count the difference of elements between both BTreeMaps by number (after a filter).


    // Merge both BTreeMaps as sets and delete equivalent items.


    // The lasting items, as they could be more or less than the usual, if less, are loss packets.


    Ok(())
}

pub fn run_binary_file(serialized_file: File) -> Result<(), Error> {
    // Deserialize captures BTreeMap.
    let captures: BTreeMap<String, Vec<CaptureResult>> =
        bincode::deserialize_from(&serialized_file).unwrap();

    Ok(())
}
