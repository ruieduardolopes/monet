use crate::capture::errors::*;
use crate::capture::results::*;
use crate::capture::packets::multimedia::rtp_h264_nal::H264NalPacket;
use crate::capture::packets::multimedia::rtp_h264_nal_fu_a::non_idr_slice::rtp_h264_nal_fu_a_non_idr_slice::H264NalFuANonIDRSlicePacket;
use crate::capture::packets::multimedia::rtp_h264_nal_fu_a::rtp_h264_nal_fu_a::H264NalFuAPacket;
use crate::capture::packets::transport::rtp::RtpPacket;

// FIXME
use crate::fragment::Fragment;
use crate::frame::*;
use crate::identification::*;

use pnet::packet::Packet;
use std::net::Ipv4Addr;

/// Handles an RTP packet, analysing its payload, returning an `CaptureError` if something
/// unexpected occurs.
///
/// # Arguments
///
/// * `packet` - A byte-slice reference to an RTP packet
pub fn handle_rtp(
    packet: &[u8],
    dest_address: Ipv4Addr,
    port: u16,
) -> Result<CaptureResult, CaptureError> {
    // Create an RTP packet from the raw data given as parameter to this function.
    let rtp_packet = RtpPacket::new(packet);

    // Considering this is an RTP packet, then handle it accordingly.
    if let Some(rtp_packet) = rtp_packet {
        // Identify this packet's payload type
        match rtp_packet.get_payload_type() {
            // If this packet's payload is H.264 AVC, then handle it accordingly.
            rtp_payload::H264AVC => handle_h264(
                rtp_packet.payload(),
                rtp_packet.get_ssrc(),
                dest_address,
                port,
                rtp_packet.get_timestamp(),
            ),
            // If this packet has a payload which type is unknown to us, return proper error.
            _ => Err(CaptureError::UnrecognizableRTPPayload),
        }
    } else {
        // If this recognition fails, return error denoting that this is not an RTP packet.
        Err(CaptureError::NotAnRTPPacket)
    }
}

/// Handles an RTP H.264 packet, analysing its payload, returning an `CaptureError` if something
/// unexpected occurs.
///
/// # Arguments
///
/// * `packet` - A byte-slice reference to an RTP H.264 packet
pub fn handle_h264(
    packet: &[u8],
    ssrc: u32,
    dest_address: Ipv4Addr,
    port: u16,
    timestamp: u32,
) -> Result<CaptureResult, CaptureError> {
    // Create an RTP's H.264 packet from the raw data given as parameter to this function.
    let h264_packet = H264NalPacket::new(packet);

    // Considering this is an RTP's H.264 packet, then handle it accordingly.
    if let Some(h264_packet) = h264_packet {
        // Identify this packet's type.
        match h264_packet.get_packet_type() {
            // If this packet's type is a Type-A Fragment Unit, then handle it accordingly.
            nal::FU_A => handle_fu_a(h264_packet.payload(), ssrc, dest_address, port, timestamp),
            // If this packet's type is an IDR Partition Type Unit, then handle it accordingly.
            nal::IDR_PARTITION => {
                println!("This is a I-frame");
                Ok(CaptureResult::Frame(Frame {
                    ssrc,
                    dest_address,
                    stream_port: port,
                    timestamp,
                    mpeg_type: MPEGType::I,
                }))
            }
            // If this packet has a type which is unknown to us, return proper error.
            _ => Err(CaptureError::UnrecognizableRTPH264PacketType),
        }
    } else {
        // If this recognition fails, return error denoting that this is a malformed packet.
        Err(CaptureError::MalformedRTPH264Packet)
    }
}

/// Handles an RTP H.264 Fragment Unit (Type-A) packet, analysing its payload, returning an `CaptureError`
/// if something unexpected occurs.
///
/// # Arguments
///
/// * `packet` - A byte-slice reference to an RTP H.264 FU-A packet
pub fn handle_fu_a(
    packet: &[u8],
    ssrc: u32,
    dest_address: Ipv4Addr,
    port: u16,
    timestamp: u32,
) -> Result<CaptureResult, CaptureError> {
    // Create an RTP's H.264 FU-A packet from the raw data given as parameter to this function.
    let fu_a_packet = H264NalFuAPacket::new(packet);

    // Considering this is an RTP's H.264 FU-A packet, then handle it accordingly.
    if let Some(fu_a_packet) = fu_a_packet {
        // Identify this packet's unit type.
        match fu_a_packet.get_unit_type() {
            // If this packet's unit type is a non-IDR slice, handle it.
            nal::NON_IDR_SLICE => {
                if fu_a_packet.get_start_bit() == 0 {
                    if fu_a_packet.get_end_bit() == 1 {
                        Ok(CaptureResult::LastFragment(Fragment {
                            ssrc,
                            dest_address,
                            stream_port: port,
                            timestamp,
                        }))
                    } else {
                        Ok(CaptureResult::Fragment(Fragment {
                            ssrc,
                            dest_address,
                            stream_port: port,
                            timestamp,
                        }))
                    }
                } else {
                    handle_slice(fu_a_packet.payload(), ssrc, dest_address, port, timestamp)
                }
            }
            // If this packet's unit type is an IDR slice (I-slice), handle it.
            nal::IDR_PARTITION => {
                if fu_a_packet.get_start_bit() == 1 {
                    Ok(CaptureResult::Frame(Frame {
                        ssrc,
                        dest_address,
                        stream_port: port,
                        timestamp,
                        mpeg_type: MPEGType::I,
                    }))
                } else {
                    Err(CaptureError::UnrecognizableRTPH264FuAUnitType)
                }
            }
            // If this packet's unit type is a SPS, handle it.
            nal::SPS => Ok(CaptureResult::SequenceParameterSet),
            // If this packet's unit type is a PPS, handle it.
            nal::PPS => Ok(CaptureResult::PictureParameterSet),
            // If this packet's unit type is unknown to us, return a proper error.
            _ => Err(CaptureError::UnrecognizableRTPH264FuAUnitType),
        }
    } else {
        // If this recognition fails, return error denoting that this is a malformed packet.
        Err(CaptureError::MalformedRTPH264FuAPacket)
    }
}

/// Handles an RTP H.264 Fragment Unit (Type-A) Slice packet, analysing its payload, returning an
/// `CaptureError` if something unexpected occurs.
///
/// # Arguments
///
/// * `packet` - A byte-slice reference to an RTP H.264 FU-A Slice packet
pub fn handle_slice(
    packet: &[u8],
    ssrc: u32,
    dest_address: Ipv4Addr,
    stream_port: u16,
    timestamp: u32,
) -> Result<CaptureResult, CaptureError> {
    // Create an RTP's H.264 FU-A Slice packet from the raw data given as parameter to this function.
    let slice_packet = H264NalFuANonIDRSlicePacket::new(packet);

    // Considering this is an RTP's H.264 FU-A Slice packet, then handle it accordingly.
    if let Some(slice_packet) = slice_packet {
        // Identify this slice type, and handle it.
        match slice_packet.get_slice_type() {
            // If this is an individual P-slice, handle it.
            slice::INDIVIDUAL_SLICE_P => Ok(CaptureResult::Frame(Frame {
                ssrc,
                dest_address,
                stream_port,
                timestamp,
                mpeg_type: MPEGType::P,
            })),
            // If this is an individual B-slice, handle it.
            slice::INDIVIDUAL_SLICE_B => Ok(CaptureResult::Frame(Frame {
                ssrc,
                dest_address,
                stream_port,
                timestamp,
                mpeg_type: MPEGType::B,
            })),
            // If this is an individual I-slice, handle it.
            slice::INDIVIDUAL_SLICE_I => Ok(CaptureResult::Frame(Frame {
                ssrc,
                dest_address,
                stream_port,
                timestamp,
                mpeg_type: MPEGType::I,
            })),
            // If this is an individual SP-slice, handle it.
            slice::INDIVIDUAL_SLICE_SP => unimplemented!(),
            // If this is an individual SI-slice, handle it.
            slice::INDIVIDUAL_SLICE_SI => unimplemented!(),
            // If this is a P-slice, handle it.
            slice::SLICE_P => Ok(CaptureResult::Frame(Frame {
                ssrc,
                dest_address,
                stream_port,
                timestamp,
                mpeg_type: MPEGType::P,
            })),
            // If this is a B-slice, handle it.
            slice::SLICE_B => Ok(CaptureResult::Frame(Frame {
                ssrc,
                dest_address,
                stream_port,
                timestamp,
                mpeg_type: MPEGType::B,
            })),
            // If this is a I-slice, handle it.
            slice::SLICE_I => Ok(CaptureResult::Frame(Frame {
                ssrc,
                dest_address,
                stream_port,
                timestamp,
                mpeg_type: MPEGType::I,
            })),
            // If this is a SP-slice, handle it.
            slice::SLICE_SP => unimplemented!(),
            // If this is a SI-slice, handle it.
            slice::SLICE_SI => unimplemented!(),
            // If this slice is none of the above, then return an error for unrecognizable slice.
            _ => Err(CaptureError::UnrecognizableRTPH264SliceType),
        }
    } else {
        // If this recognition fails, return error denoting that this is a malformed slice packet.
        Err(CaptureError::MalformedRTPH264SlicePacket)
    }
}
