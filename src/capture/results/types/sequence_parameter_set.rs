use crate::capture::results::CaptureResult;

use std::net::Ipv4Addr;

#[derive(Deserialize, Serialize)]
pub struct SequenceParameterSetResult {
    pub destination_address: Ipv4Addr,
    pub timestamp: i64,
}

impl SequenceParameterSetResult {
    pub fn new(destination_address: Ipv4Addr, timestamp: i64) -> Self {
        SequenceParameterSetResult {
            destination_address,
            timestamp,
        }
    }

    pub fn launch(destination_address: Ipv4Addr, timestamp: i64) -> CaptureResult {
        CaptureResult::SequenceParameterSet(SequenceParameterSetResult::new(
            destination_address,
            timestamp,
        ))
    }
}