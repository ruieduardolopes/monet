pub mod types;

use core::fmt;
use std::fmt::Display;
use types::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CaptureResult {
    // Result, packet_length and sequence_number from RTP.
    Frame(FrameResult, u16, u16),
    Fragment(FragmentResult, u16, u16),
    LastFragment(LastFragmentResult, u16, u16),
    PictureParameterSet(PictureParameterSetResult, u16, u16),
    SequenceParameterSet(SequenceParameterSetResult, u16, u16),
    Stream(StreamResult, u16),
    Other(OtherResult, u16),
}

impl Display for CaptureResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureResult::Frame(frame, packet_length, sequence_number) => write!(
                f,
                "[monet] frame {} {} {} {} {} {} {} {}",
                frame.container.timestamp,
                frame.container.dest_address,
                frame.container.stream_port,
                frame.container.ssrc,
                frame.container.mpeg_type,
                packet_length,
                sequence_number,
                frame.timestamp
            ),
            CaptureResult::Fragment(fragment, packet_length, sequence_number) => write!(
                f,
                "[monet] fragment {} {} {} {} not-last {} {} {}",
                fragment.container.timestamp,
                fragment.container.dest_address,
                fragment.container.stream_port,
                fragment.container.ssrc,
                packet_length,
                sequence_number,
                fragment.timestamp
            ),
            CaptureResult::LastFragment(fragment, packet_length, sequence_number) => write!(
                f,
                "[monet] fragment {} {} {} {} last {} {} {}",
                fragment.container.timestamp,
                fragment.container.dest_address,
                fragment.container.stream_port,
                fragment.container.ssrc,
                packet_length,
                sequence_number,
                fragment.timestamp
            ),
            CaptureResult::SequenceParameterSet(sps, packet_length, sequence_number) => write!(
                f,
                "[monet] sps {} {} {} {}",
                sps.destination_address, packet_length, sequence_number, sps.timestamp
            ),
            CaptureResult::PictureParameterSet(pps, packet_length, sequence_number) => write!(
                f,
                "[monet] pps {} {} {} {}",
                pps.destination_address, packet_length, sequence_number, pps.timestamp
            ),
            CaptureResult::Stream(stream, packet_length) => write!(
                f,
                "[monet] stream {} {} {} {} {} {}",
                stream.last_known_timestamp,
                stream.destination_address,
                stream.destination_port,
                stream.ssrc,
                packet_length,
                stream.timestamp
            ),
            _ => Ok(()),
        }
    }
}

impl From<CaptureResult> for &str {
    fn from(result: CaptureResult) -> Self {
        match result {
            CaptureResult::Frame(_, _, _) => "h264",
            CaptureResult::Fragment(_, _, _) => "h264",
            CaptureResult::LastFragment(_, _, _) => "h264",
            CaptureResult::PictureParameterSet(_, _, _) => "h264",
            CaptureResult::SequenceParameterSet(_, _, _) => "h264",
            CaptureResult::Stream(_, _) => "h264",
            _ => "",
        }
    }
}
