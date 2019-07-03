// (c) 2019 Rui Lopes

//! Module responsible for managing the analysis of OSI's model Layer III payloads, retrieving
//! Layer III packets to respective handlers if the packet has tunnel headers, or Layer IV packets
//! to transport handlers, in the `transport` module.

use crate::capture::{transport, CaptureResult};
use crate::error::CaptureError;

use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use std::net::Ipv4Addr;

/// Handles a Layer III IPv4 frame, analysing its payload, returning an `CaptureError` if something
/// unexpected occurs.
///
/// # Arguments
///
/// * `packet` - A `libpnet`'s `EthernetPacket` reference to an Ethernet packet
pub fn handle_ipv4_packet(packet: &EthernetPacket) -> Result<CaptureResult, CaptureError> {
    // Read the received packet's payload as an IPv4 packet.
    let ipv4_packet = Ipv4Packet::new(packet.payload());

    // Considering this is an IPv4 packet, handle it.
    if let Some(ipv4_packet) = ipv4_packet {
        // Test if this is a tunnel header.
        match ipv4_packet.get_next_level_protocol() {
            // This payload corresponds to an IPv4 packet.
            IpNextHeaderProtocols::Ipv4 => {
                handle_ipv4_tunneled_packet(ipv4_packet.payload(), ipv4_packet.get_destination())
            }
            // This payload corresponds to an IPv6 packet.
            IpNextHeaderProtocols::Ipv6 => {
                handle_ipv6_tunneled_packet(ipv4_packet.payload(), ipv4_packet.get_destination())
            }
            // This payload either is a transport layer packet to be handled or is a malformed packet
            _ => transport::handle(
                ipv4_packet.get_next_level_protocol(),
                ipv4_packet.payload(),
                ipv4_packet.get_destination(),
            ),
        }
    } else {
        return Err(CaptureError::UnrecognizableIPv4Packet);
    }
}

/// Handles a tunneled Layer III IPv4 frame, analysing its payload, returning an `CaptureError` if
/// something unexpected occurs.
///
/// # Arguments
///
/// * `packet` - A slice representing a tunneled IPv4 packet
pub fn handle_ipv4_tunneled_packet(
    packet: &[u8],
    last_dest_address: Ipv4Addr,
) -> Result<CaptureResult, CaptureError> {
    // Read the received packet's payload as an IPv4 packet.
    let ipv4_packet = Ipv4Packet::new(packet);

    // Considering this is an IPv4 packet, handle it.
    if let Some(ipv4_packet) = ipv4_packet {
        // Test if this is a tunnel header.
        match ipv4_packet.get_next_level_protocol() {
            // This payload corresponds to an IPv4 packet.
            IpNextHeaderProtocols::Ipv4 => {
                handle_ipv4_tunneled_packet(ipv4_packet.payload(), ipv4_packet.get_destination())
            }
            // This payload corresponds to an IPv6 packet.
            IpNextHeaderProtocols::Ipv6 => {
                handle_ipv6_tunneled_packet(ipv4_packet.payload(), ipv4_packet.get_destination())
            }
            // This payload either is a transport layer packet to be handled or is a malformed packet
            _ => transport::handle(
                ipv4_packet.get_next_level_protocol(),
                ipv4_packet.payload(),
                ipv4_packet.get_destination(),
            ),
        }
    } else {
        return Err(CaptureError::UnrecognizableIPv4Packet);
    }
}

/// Handles a Layer III IPv6 frame, analysing its payload, returning an `CaptureError` if something
/// unexpected occurs.
///
/// # Arguments
///
/// * `packet` - A `libpnet`'s `EthernetPacket` reference to a packet (Ethernet or IPv4/6 if tunneled)
pub fn handle_ipv6_packet(packet: &EthernetPacket) -> Result<CaptureResult, CaptureError> {
    // Read the received packet's payload as an IPv6 packet.
    let ipv6_packet = Ipv6Packet::new(packet.payload());

    // Considering this is an IPv6 packet, handle it.
    if let Some(ipv6_packet) = ipv6_packet {
        // Test if this is a tunnel header.
        match ipv6_packet.get_next_header() {
            // This payload corresponds to an IPv4 packet.
            IpNextHeaderProtocols::Ipv4 => {
                handle_ipv4_tunneled_packet(ipv6_packet.payload(), Ipv4Addr::new(0, 0, 0, 0))
            }
            // This payload corresponds to an IPv6 packet.
            IpNextHeaderProtocols::Ipv6 => {
                handle_ipv6_tunneled_packet(ipv6_packet.payload(), Ipv4Addr::new(0, 0, 0, 0))
            }
            // This payload either is a transport layer packet to be handled or is a malformed packet
            _ => transport::handle(
                ipv6_packet.get_next_header(),
                ipv6_packet.payload(),
                Ipv4Addr::new(0, 0, 0, 0),
            ),
        }
    } else {
        return Err(CaptureError::UnrecognizableIPv6Packet);
    }
}

/// Handles a tunneled Layer III IPv6 frame, analysing its payload, returning an `CaptureError` if
/// something unexpected occurs.
///
/// # Arguments
///
/// * `packet` - A slice representing a tunneled IPv6 packet
pub fn handle_ipv6_tunneled_packet(
    packet: &[u8],
    last_dest_address: Ipv4Addr,
) -> Result<CaptureResult, CaptureError> {
    // Read the received packet's payload as an IPv6 packet.
    let ipv6_packet = Ipv6Packet::new(packet);

    // Considering this is an IPv6 packet, handle it.
    if let Some(ipv6_packet) = ipv6_packet {
        // Test if this is a tunnel header.
        match ipv6_packet.get_next_header() {
            // This payload corresponds to an IPv4 packet.
            IpNextHeaderProtocols::Ipv4 => {
                handle_ipv4_tunneled_packet(ipv6_packet.payload(), last_dest_address)
            }
            // This payload corresponds to an IPv6 packet.
            IpNextHeaderProtocols::Ipv6 => {
                handle_ipv6_tunneled_packet(ipv6_packet.payload(), last_dest_address)
            }
            // This payload either is a transport layer packet to be handled or is a malformed packet
            _ => transport::handle(
                ipv6_packet.get_next_header(),
                ipv6_packet.payload(),
                last_dest_address,
            ),
        }
    } else {
        return Err(CaptureError::UnrecognizableIPv6Packet);
    }
}
