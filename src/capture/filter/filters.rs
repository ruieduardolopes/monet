use crate::capture::results::CaptureResult;

use regex::Regex;

pub enum FilterValues {
    H264,
    TCP,
    UDP,
}

impl From<&FilterValues> for &str {
    fn from(filter: &FilterValues) -> Self {
        match filter {
            FilterValues::H264 => "h264",
            FilterValues::TCP => "tcp",
            FilterValues::UDP => "udp",
        }
    }
}