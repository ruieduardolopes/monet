use crate::capture::osi::transport::multimedia::structs::fragment::Fragment;
use crate::capture::osi::transport::multimedia::structs::frame::Frame;

use std::net::Ipv4Addr;
use core::fmt;
use std::fmt::Display;

#[derive(Deserialize, Serialize)]
pub enum CaptureResult {
    Frame(Frame, i64),
    Fragment(Fragment, i64),
    LastFragment(Fragment, i64),
    SequenceParameterSet(Ipv4Addr, i64),
    PictureParameterSet(Ipv4Addr, i64),
    Stream(u32, Ipv4Addr, u16, u32, i64), // ssrc, port, last timestamp
    Other(i64),
}

impl Display for CaptureResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureResult::Frame(frame, timestamp) => write!(
                f,
                "[monet] frame {} {} {} {} {} {}",
                frame.timestamp,
                frame.dest_address,
                frame.stream_port,
                frame.ssrc,
                frame.mpeg_type,
                timestamp
            ),
            CaptureResult::Fragment(fragment, timestamp) => write!(
                f,
                "[monet] {} {} {} {} not-last {}",
                fragment.timestamp,
                fragment.dest_address,
                fragment.stream_port,
                fragment.ssrc,
                timestamp
            ),
            CaptureResult::LastFragment(fragment, timestamp) => write!(
                f,
                "[monet] {} {} {} {} last {}",
                fragment.timestamp,
                fragment.dest_address,
                fragment.stream_port,
                fragment.ssrc,
                timestamp
            ),
            CaptureResult::SequenceParameterSet(dst_address, timestamp) => {
                write!(f, "[monet] sps {} {}", dst_address, timestamp)
            }
            CaptureResult::PictureParameterSet(dst_address, timestamp) => {
                write!(f, "[monet] pps {} {}", dst_address, timestamp)
            }
            CaptureResult::Stream(
                ssrc,
                destination_address,
                destination_port,
                last_known_timestamp,
                timestamp
            ) => write!(
                f,
                "[monet] stream {} {} {} {} {}",
                last_known_timestamp,
                destination_address,
                destination_port,
                ssrc,
                timestamp
            ),
            _ => Ok(()),
        }
    }
}
