use std::net::Ipv4Addr;

pub enum CaptureResult {
    Frame(Frame),
    Fragment(Fragment),
    LastFragment(Fragment),
    SequenceParameterSet,
    PictureParameterSet,
    Stream(u32, Ipv4Addr, u16, u32), // ssrc, port, last timestamp
    Other,
}