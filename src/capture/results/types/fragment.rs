use crate::capture::osi::transport::multimedia::structs::fragment::Fragment;
use crate::capture::results::CaptureResult;

use std::net::Ipv4Addr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FragmentResult {
    pub container: Fragment,
    pub timestamp: i64,
}

impl FragmentResult {
    pub fn new(container: Fragment, timestamp: i64) -> Self {
        FragmentResult {
            container,
            timestamp,
        }
    }

    pub fn launch(
        ssrc: u32,
        dest_address: Ipv4Addr,
        stream_port: u16,
        fragment_timestamp: u32,
        timestamp: i64,
    ) -> CaptureResult {
        CaptureResult::Fragment(FragmentResult::new(
            Fragment::new(ssrc, dest_address, stream_port, fragment_timestamp),
            timestamp,
        ))
    }
}
