// (c) 2019 Rui Lopes

//! Module responsible for managing the analysis of OSI's model Layer II payloads, retrieving
//! Layer III packets to respective handlers, in the `network` module.

use crate::capture::errors::*;
use crate::capture::osi::*;
use crate::capture::results::*;

use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

/// Handles a Layer II Ethernet frame, analysing its payload, returning an `CaptureError` if something
/// unexpected occurs.
///
/// # Arguments
///
/// * `packet` - A `libpnet`'s `EthernetPacket` reference to a Layer II Ethernet packet
pub fn handle_ethernet_frame(packet: &EthernetPacket) -> Result<CaptureResult, CaptureError> {
    // Match packet's payload with a Layer III protocol.
    match packet.get_ethertype() {
        // This payload corresponds to an IPv4 packet.
        EtherTypes::Ipv4 => network::handle_ipv4_packet(packet),
        // This payload corresponds to an IPv6 packet.
        EtherTypes::Ipv6 => network::handle_ipv6_packet(packet),
        // This payload corresponds to an unknown packet.
        _ => Err(CaptureError::UnrecognizableEthernetPayload),
    }
}
