use crate::capture::osi::transport::multimedia::structs::fragment::Fragment;
use crate::capture::results::CaptureResult;

use std::net::Ipv4Addr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LastFragmentResult {
    pub container: Fragment,
    pub timestamp: i64,
}

impl LastFragmentResult {
    pub fn new(container: Fragment, timestamp: i64) -> Self {
        LastFragmentResult {
            container,
            timestamp,
        }
    }

    pub fn launch(
        ssrc: u32,
        dest_address: Ipv4Addr,
        stream_port: u16,
        packet_length: u16,
        sequence_number: u16,
        fragment_timestamp: u32,
        timestamp: i64,
    ) -> CaptureResult {
        CaptureResult::LastFragment(
            LastFragmentResult::new(
                Fragment::new(ssrc, dest_address, stream_port, fragment_timestamp),
                timestamp,
            ),
            packet_length,
            sequence_number,
        )
    }
}
