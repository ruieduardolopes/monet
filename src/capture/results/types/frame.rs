use crate::capture::osi::transport::multimedia::structs::frame::*;
use crate::capture::results::CaptureResult;

use std::net::Ipv4Addr;

#[derive(Deserialize, Serialize)]
pub struct FrameResult {
    pub container: Frame,
    pub timestamp: i64,
}

impl FrameResult {
    pub fn new(container: Frame, timestamp: i64) -> Self {
        FrameResult {
            container,
            timestamp,
        }
    }

    pub fn launch(
        ssrc: u32,
        dest_address: Ipv4Addr,
        stream_port: u16,
        frame_timestamp: u32,
        mpeg_type: MPEGType,
        timestamp: i64,
    ) -> CaptureResult {
        CaptureResult::Frame(FrameResult::new(
            Frame::new(ssrc, dest_address, stream_port, frame_timestamp, mpeg_type),
            timestamp,
        ))
    }
}
