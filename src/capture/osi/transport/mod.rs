// (c) 2019 Rui Lopes

pub mod control;
pub mod multimedia;

use crate::capture::errors::*;
use crate::capture::results::*;

use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::net::Ipv4Addr;

/// Handles a TCP packet, analysing its payload, returning an `CaptureError` if something
/// unexpected occurs.
///
/// > This method is not yet implemented!
///
/// # Arguments
///
/// * `packet` - A byte-slice reference to a TCP packet
fn handle_tcp(packet: &[u8], dest_address: Ipv4Addr) -> Result<(), CaptureError> {
    // Create the TCP packet from the raw data given as parameter to this function.
    let tcp_packet = TcpPacket::new(packet);

    // Considering this is a TCP packet, handle its payload.
    if let Some(_tcp_packet) = tcp_packet {
        // Right now there is no usage to this context.
        Ok(())
    } else {
        // If an error occurs, then we consider that this TCP packet was malformed.
        Err(CaptureError::MalformedTCPPacket)
    }
}

/// Handles a UDP packet, analysing its payload, returning an `CaptureError` if something
/// unexpected occurs.
///
/// > This method is not yet implemented!
///
/// # Arguments
///
/// * `packet` - A byte-slice reference to a UDP packet
fn handle_udp(packet: &[u8], dest_address: Ipv4Addr) -> Result<CaptureResult, CaptureError> {
    // Create the UDP packet from the raw data given as parameter to this function.
    let udp_packet = UdpPacket::new(packet);

    // Considering this is a UDP packet, handle its payload.
    if let Some(udp_packet) = udp_packet {
        let result: Result<CaptureResult, CaptureError>;
        // Handle this UDP packet's payload as RTP's.
        result = multimedia::handle_rtp(
            udp_packet.payload(),
            dest_address,
            udp_packet.get_destination(),
        );
        if result.is_ok() {
            return result;
        }
        control::handle_rtcp(
            udp_packet.payload(),
            dest_address,
            udp_packet.get_destination(),
        )
    } else {
        // If an error occurs, then we consider that this UDP packet was malformed.
        Err(CaptureError::MalformedUDPPacket)
    }
}

/// Handles a Layer IV frame, analysing its payload, returning an `CaptureError` if something
/// unexpected occurs.
///
/// # Arguments
///
/// * `protocol` - A `libpnet`'s `EthernetPacket` reference to a packet (Ethernet or IPv4/6 if tunneled)
/// * `packet` - A `libpnet`'s `EthernetPacket` reference to a packet (Ethernet or IPv4/6 if tunneled)
pub fn handle(
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
    dest_address: Ipv4Addr,
) -> Result<CaptureResult, CaptureError> {
    // Verify the inner transport protocol to handle it accordingly.
    match protocol {
        // If this packet is a TCP, handle it as TCP.
        IpNextHeaderProtocols::Tcp => {
            handle_tcp(packet, dest_address).unwrap();
            Ok(CaptureResult::Other)
        }
        // If this packet is a UDP, handle it as UDP.
        IpNextHeaderProtocols::Udp => match handle_udp(packet, dest_address) {
            Ok(packet) => Ok(packet),
            Err(_error) => Err(_error),
        },
        // If this packet is unrecognizable, then return the proper error.
        _ => Err(CaptureError::UnrecognizableTransportPacket),
    }
}
