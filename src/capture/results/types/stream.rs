use crate::capture::results::CaptureResult;

use std::net::Ipv4Addr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamResult {
    pub ssrc: u32,
    pub destination_address: Ipv4Addr,
    pub destination_port: u16,
    pub last_known_timestamp: u32,
    pub timestamp: i64,
}

impl StreamResult {
    pub fn new(
        ssrc: u32,
        destination_address: Ipv4Addr,
        destination_port: u16,
        last_known_timestamp: u32,
        timestamp: i64,
    ) -> Self {
        StreamResult {
            ssrc,
            destination_address,
            destination_port,
            last_known_timestamp,
            timestamp,
        }
    }

    pub fn launch(
        ssrc: u32,
        destination_address: Ipv4Addr,
        destination_port: u16,
        last_known_timestamp: u32,
        timestamp: i64,
    ) -> CaptureResult {
        CaptureResult::Stream(StreamResult::new(
            ssrc,
            destination_address,
            destination_port,
            last_known_timestamp,
            timestamp,
        ))
    }
}
