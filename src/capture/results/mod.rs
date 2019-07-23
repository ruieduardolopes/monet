pub mod types;

use core::fmt;
use std::fmt::Display;
use types::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CaptureResult {
    Frame(FrameResult),
    Fragment(FragmentResult),
    LastFragment(LastFragmentResult),
    PictureParameterSet(PictureParameterSetResult),
    SequenceParameterSet(SequenceParameterSetResult),
    Stream(StreamResult),
    Other(OtherResult),
}

impl Display for CaptureResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureResult::Frame(frame) => write!(
                f,
                "[monet] frame {} {} {} {} {} {}",
                frame.container.timestamp,
                frame.container.dest_address,
                frame.container.stream_port,
                frame.container.ssrc,
                frame.container.mpeg_type,
                frame.timestamp
            ),
            CaptureResult::Fragment(fragment) => write!(
                f,
                "[monet] {} {} {} {} not-last {}",
                fragment.container.timestamp,
                fragment.container.dest_address,
                fragment.container.stream_port,
                fragment.container.ssrc,
                fragment.timestamp
            ),
            CaptureResult::LastFragment(fragment) => write!(
                f,
                "[monet] {} {} {} {} last {}",
                fragment.container.timestamp,
                fragment.container.dest_address,
                fragment.container.stream_port,
                fragment.container.ssrc,
                fragment.timestamp
            ),
            CaptureResult::SequenceParameterSet(sps) => write!(
                f,
                "[monet] sps {} {}",
                sps.destination_address, sps.timestamp
            ),
            CaptureResult::PictureParameterSet(pps) => write!(
                f,
                "[monet] pps {} {}",
                pps.destination_address, pps.timestamp
            ),
            CaptureResult::Stream(stream) => write!(
                f,
                "[monet] stream {} {} {} {} {}",
                stream.last_known_timestamp,
                stream.destination_address,
                stream.destination_port,
                stream.ssrc,
                stream.timestamp
            ),
            _ => Ok(()),
        }
    }
}
