use crate::capture::results::CaptureResult;

use std::net::Ipv4Addr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PictureParameterSetResult {
    pub destination_address: Ipv4Addr,
    pub timestamp: i64,
}

impl PictureParameterSetResult {
    pub fn new(destination_address: Ipv4Addr, timestamp: i64) -> Self {
        PictureParameterSetResult {
            destination_address,
            timestamp,
        }
    }

    pub fn launch(
        destination_address: Ipv4Addr,
        packet_length: u16,
        sequence_number: u16,
        timestamp: i64,
    ) -> CaptureResult {
        CaptureResult::PictureParameterSet(
            PictureParameterSetResult::new(destination_address, timestamp),
            packet_length,
            sequence_number,
        )
    }
}
