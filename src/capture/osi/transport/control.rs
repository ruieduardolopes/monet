use crate::capture::CaptureResult;
use crate::error::CaptureError;
use crate::identification::rtcp_type::*;
use crate::packet::control::rtcp::RtcpPacket;
use crate::packet::control::types::rr::RtcpRrPacket;
use crate::packet::control::types::sr::RtcpSrPacket;
use pnet::packet::Packet;
use std::net::Ipv4Addr;

pub fn handle_rtcp(
    packet: &[u8],
    dest_address: Ipv4Addr,
    port: u16,
) -> Result<CaptureResult, CaptureError> {
    let rtcp_packet = RtcpPacket::new(packet);

    if let Some(rtcp_packet) = rtcp_packet {
        match rtcp_packet.get_packet_type() {
            SENDER_REPORT => {
                let sender_report = RtcpSrPacket::new(rtcp_packet.payload());

                if let Some(sender_report) = sender_report {
                    return Ok(CaptureResult::Stream(
                        sender_report.get_ssrc(),
                        dest_address,
                        port,
                        sender_report.get_rtp_timestamp(),
                    ));
                }
                return Err(CaptureError::MalformedRTCPSRPacket);
            }
            RECEIVER_REPORT => {
                return Err(CaptureError::IrrelevantRTCPPacket);
            }
            BYE => {
                return Err(CaptureError::IrrelevantRTCPPacket);
            }
            APP => {
                return Err(CaptureError::IrrelevantRTCPPacket);
            }
            SOURCE_DESCRIPTION => {
                return Err(CaptureError::IrrelevantRTCPPacket);
            }
            _ => Err(CaptureError::NotAnRTCPPacket),
        }
    } else {
        Err(CaptureError::NotAnRTCPPacket)
    }
}
