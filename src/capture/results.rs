use std::net::Ipv4Addr;

use crate::capture::osi::transport::multimedia::structs::fragment::Fragment;
use crate::capture::osi::transport::multimedia::structs::frame::Frame;
use core::fmt;
use std::fmt::Display;

#[derive(Deserialize, Serialize)]
pub enum CaptureResult {
    Frame(Frame),
    Fragment(Fragment),
    LastFragment(Fragment),
    SequenceParameterSet,
    PictureParameterSet,
    Stream(u32, Ipv4Addr, u16, u32), // ssrc, port, last timestamp
    Other,
}

impl Display for CaptureResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureResult::Frame(frame) => write!(
                f,
                "[monet] frame {} {} {} {} {} {}",
                frame.timestamp,
                frame.dest_address,
                frame.stream_port,
                frame.ssrc,
                frame.mpeg_type,
                time::now_utc().to_timespec().sec
            ),
            CaptureResult::Fragment(fragment) => write!(
                f,
                "[monet] {} {} {} {} not-last {}",
                fragment.timestamp,
                fragment.dest_address,
                fragment.stream_port,
                fragment.ssrc,
                time::now_utc().to_timespec().sec
            ),
            CaptureResult::LastFragment(fragment) => write!(
                f,
                "[monet] {} {} {} {} last {}",
                fragment.timestamp,
                fragment.dest_address,
                fragment.stream_port,
                fragment.ssrc,
                time::now_utc().to_timespec().sec
            ),
            CaptureResult::SequenceParameterSet => {
                write!(f, "[monet] sps {}", time::now_utc().to_timespec().sec)
            }
            CaptureResult::PictureParameterSet => {
                write!(f, "[monet] pps {}", time::now_utc().to_timespec().sec)
            }
            CaptureResult::Stream(
                ssrc,
                destination_address,
                destination_port,
                last_known_timestamp,
            ) => write!(
                f,
                "[monet] stream {} {} {} {} {}",
                last_known_timestamp,
                destination_address,
                destination_port,
                ssrc,
                time::now_utc().to_timespec().sec
            ),
            _ => Ok(()),
        }
    }
}
