use crate::analysis;
use crate::capture::interfaces::*;
use crate::capture::results::CaptureResult;
use crate::structs::node::Node;

use crossbeam_utils::thread;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Error;

pub fn init(ingress: String, egress: String, filter: Regex) -> Result<(), Error> {
    unimplemented!();

    Ok(())
}
