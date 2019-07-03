use pnet_macros_support::types::*;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct RtcpSrReportBlockPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableRtcpSrReportBlockPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> RtcpSrReportBlockPacket<'a> {
    /// Constructs a new RtcpSrReportBlockPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<RtcpSrReportBlockPacket<'p>> {
        if packet.len() >= RtcpSrReportBlockPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpSrReportBlockPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new RtcpSrReportBlockPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the RtcpSrReportBlockPacket will
    /// own its own data and the underlying buffer will be dropped when the RtcpSrReportBlockPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<RtcpSrReportBlockPacket<'static>> {
        if packet.len() >= RtcpSrReportBlockPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpSrReportBlockPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a RtcpSrReportBlockPacket to a RtcpSrReportBlockPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpSrReportBlockPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpSrReportBlockPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a RtcpSrReportBlockPacket to a RtcpSrReportBlockPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpSrReportBlockPacket<'a> {
        RtcpSrReportBlockPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        24
    }
    /// The size (in bytes) of a RtcpSrReportBlock instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpSrReportBlock) -> usize {
        24 + _packet.payload.len()
    }
    /// Get the ssrc field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ssrc(&self) -> u32be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the fraction_lost field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_fraction_lost(&self) -> u8 {
        let _self = self;
        let co = 4;
        (_self.packet[co] as u8)
    }
    /// Get the cummulative_number_of_packets_lost field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_cummulative_number_of_packets_lost(&self) -> u24be {
        let _self = self;
        let co = 5;
        let b0 = ((_self.packet[co + 0] as u24be) << 16) as u24be;
        let b1 = ((_self.packet[co + 1] as u24be) << 8) as u24be;
        let b2 = (_self.packet[co + 2] as u24be) as u24be;
        b0 | b1 | b2
    }
    /// Get the extended_highest_sequence_number_received field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_extended_highest_sequence_number_received(&self) -> u32be {
        let _self = self;
        let co = 8;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the interarrival_jitter field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_interarrival_jitter(&self) -> u32be {
        let _self = self;
        let co = 12;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the lsr field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_lsr(&self) -> u32be {
        let _self = self;
        let co = 16;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the dlsr field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_dlsr(&self) -> u32be {
        let _self = self;
        let co = 20;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
}
impl<'a> MutableRtcpSrReportBlockPacket<'a> {
    /// Constructs a new MutableRtcpSrReportBlockPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableRtcpSrReportBlockPacket<'p>> {
        if packet.len() >= MutableRtcpSrReportBlockPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpSrReportBlockPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableRtcpSrReportBlockPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableRtcpSrReportBlockPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableRtcpSrReportBlockPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableRtcpSrReportBlockPacket<'static>> {
        if packet.len() >= MutableRtcpSrReportBlockPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpSrReportBlockPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableRtcpSrReportBlockPacket to a RtcpSrReportBlockPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpSrReportBlockPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpSrReportBlockPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableRtcpSrReportBlockPacket to a RtcpSrReportBlockPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpSrReportBlockPacket<'a> {
        RtcpSrReportBlockPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        24
    }
    /// The size (in bytes) of a RtcpSrReportBlock instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpSrReportBlock) -> usize {
        24 + _packet.payload.len()
    }
    /// Populates a RtcpSrReportBlockPacket using a RtcpSrReportBlock structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &RtcpSrReportBlock) {
        let _self = self;
        _self.set_ssrc(packet.ssrc);
        _self.set_fraction_lost(packet.fraction_lost);
        _self.set_cummulative_number_of_packets_lost(packet.cummulative_number_of_packets_lost);
        _self.set_extended_highest_sequence_number_received(
            packet.extended_highest_sequence_number_received,
        );
        _self.set_interarrival_jitter(packet.interarrival_jitter);
        _self.set_lsr(packet.lsr);
        _self.set_dlsr(packet.dlsr);
        _self.set_payload(&packet.payload);
    }
    /// Get the ssrc field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ssrc(&self) -> u32be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the fraction_lost field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_fraction_lost(&self) -> u8 {
        let _self = self;
        let co = 4;
        (_self.packet[co] as u8)
    }
    /// Get the cummulative_number_of_packets_lost field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_cummulative_number_of_packets_lost(&self) -> u24be {
        let _self = self;
        let co = 5;
        let b0 = ((_self.packet[co + 0] as u24be) << 16) as u24be;
        let b1 = ((_self.packet[co + 1] as u24be) << 8) as u24be;
        let b2 = (_self.packet[co + 2] as u24be) as u24be;
        b0 | b1 | b2
    }
    /// Get the extended_highest_sequence_number_received field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_extended_highest_sequence_number_received(&self) -> u32be {
        let _self = self;
        let co = 8;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the interarrival_jitter field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_interarrival_jitter(&self) -> u32be {
        let _self = self;
        let co = 12;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the lsr field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_lsr(&self) -> u32be {
        let _self = self;
        let co = 16;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the dlsr field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_dlsr(&self) -> u32be {
        let _self = self;
        let co = 20;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Set the ssrc field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_ssrc(&mut self, val: u32be) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the fraction_lost field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_fraction_lost(&mut self, val: u8) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the cummulative_number_of_packets_lost field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_cummulative_number_of_packets_lost(&mut self, val: u24be) {
        let _self = self;
        let co = 5;
        _self.packet[co + 0] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 1] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 2] = (val) as u8;
    }
    /// Set the extended_highest_sequence_number_received field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_extended_highest_sequence_number_received(&mut self, val: u32be) {
        let _self = self;
        let co = 8;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the interarrival_jitter field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_interarrival_jitter(&mut self, val: u32be) {
        let _self = self;
        let co = 12;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the lsr field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_lsr(&mut self, val: u32be) {
        let _self = self;
        let co = 16;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the dlsr field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_dlsr(&mut self, val: u32be) {
        let _self = self;
        let co = 20;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 24;
        unsafe {
            copy_nonoverlapping(
                vals[..].as_ptr(),
                _self.packet[current_offset..].as_mut_ptr(),
                vals.len(),
            )
        }
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for RtcpSrReportBlockPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        24
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableRtcpSrReportBlockPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        24
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableRtcpSrReportBlockPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
        &mut self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 24;
        if _self.packet.len() <= start {
            return &mut [];
        }
        &mut _self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for MutableRtcpSrReportBlockPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 24;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for RtcpSrReportBlockPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 24;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `RtcpSrReportBlockPacket`s
pub struct RtcpSrReportBlockIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for RtcpSrReportBlockIterable<'a> {
    type Item = RtcpSrReportBlockPacket<'a>;
    fn next(&mut self) -> Option<RtcpSrReportBlockPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = RtcpSrReportBlockPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for RtcpSrReportBlockPacket<'p> {
    type T = RtcpSrReportBlock;
    #[inline]
    fn from_packet(&self) -> RtcpSrReportBlock {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpSrReportBlock {
            ssrc: _self.get_ssrc(),
            fraction_lost: _self.get_fraction_lost(),
            cummulative_number_of_packets_lost: _self.get_cummulative_number_of_packets_lost(),
            extended_highest_sequence_number_received: _self
                .get_extended_highest_sequence_number_received(),
            interarrival_jitter: _self.get_interarrival_jitter(),
            lsr: _self.get_lsr(),
            dlsr: _self.get_dlsr(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableRtcpSrReportBlockPacket<'p> {
    type T = RtcpSrReportBlock;
    #[inline]
    fn from_packet(&self) -> RtcpSrReportBlock {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpSrReportBlock {
            ssrc: _self.get_ssrc(),
            fraction_lost: _self.get_fraction_lost(),
            cummulative_number_of_packets_lost: _self.get_cummulative_number_of_packets_lost(),
            extended_highest_sequence_number_received: _self
                .get_extended_highest_sequence_number_received(),
            interarrival_jitter: _self.get_interarrival_jitter(),
            lsr: _self.get_lsr(),
            dlsr: _self.get_dlsr(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for RtcpSrReportBlockPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "RtcpSrReportBlockPacket {{ ssrc : {:?}, fraction_lost : {:?}, cummulative_number_of_packets_lost : {:?}, extended_highest_sequence_number_received : {:?}, interarrival_jitter : {:?}, lsr : {:?}, dlsr : {:?},  }}"
               , _self . get_ssrc (  ) , _self . get_fraction_lost (  ) ,
               _self . get_cummulative_number_of_packets_lost (  ) , _self .
               get_extended_highest_sequence_number_received (  ) , _self .
               get_interarrival_jitter (  ) , _self . get_lsr (  ) , _self .
               get_dlsr (  ))
    }
}
impl<'p> ::std::fmt::Debug for MutableRtcpSrReportBlockPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableRtcpSrReportBlockPacket {{ ssrc : {:?}, fraction_lost : {:?}, cummulative_number_of_packets_lost : {:?}, extended_highest_sequence_number_received : {:?}, interarrival_jitter : {:?}, lsr : {:?}, dlsr : {:?},  }}"
               , _self . get_ssrc (  ) , _self . get_fraction_lost (  ) ,
               _self . get_cummulative_number_of_packets_lost (  ) , _self .
               get_extended_highest_sequence_number_received (  ) , _self .
               get_interarrival_jitter (  ) , _self . get_lsr (  ) , _self .
               get_dlsr (  ))
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct RtcpSrReportBlock {
    pub ssrc: u32be,
    pub fraction_lost: u8,
    pub cummulative_number_of_packets_lost: u24be,
    pub extended_highest_sequence_number_received: u32be,
    pub interarrival_jitter: u32be,
    pub lsr: u32be,
    pub dlsr: u32be,
    pub payload: Vec<u8>,
}
