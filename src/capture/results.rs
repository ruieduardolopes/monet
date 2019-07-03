use std::net::Ipv4Addr;

use crate::capture::osi::transport::multimedia::structs::fragment::Fragment;
use crate::capture::osi::transport::multimedia::structs::frame::Frame;

pub enum CaptureResult {
    Frame(Frame),
    Fragment(Fragment),
    LastFragment(Fragment),
    SequenceParameterSet,
    PictureParameterSet,
    Stream(u32, Ipv4Addr, u16, u32), // ssrc, port, last timestamp
    Other,
}
