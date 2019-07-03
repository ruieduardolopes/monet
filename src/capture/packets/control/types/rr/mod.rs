use pnet_macros_support::types::*;

pub mod rtcp_rr_report_block;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct RtcpRrPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableRtcpRrPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> RtcpRrPacket<'a> {
    /// Constructs a new RtcpRrPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<RtcpRrPacket<'p>> {
        if packet.len() >= RtcpRrPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpRrPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new RtcpRrPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the RtcpRrPacket will
    /// own its own data and the underlying buffer will be dropped when the RtcpRrPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<RtcpRrPacket<'static>> {
        if packet.len() >= RtcpRrPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpRrPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a RtcpRrPacket to a RtcpRrPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpRrPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpRrPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a RtcpRrPacket to a RtcpRrPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpRrPacket<'a> {
        RtcpRrPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        4
    }
    /// The size (in bytes) of a RtcpRr instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpRr) -> usize {
        4 + _packet.payload.len()
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
}
impl<'a> MutableRtcpRrPacket<'a> {
    /// Constructs a new MutableRtcpRrPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableRtcpRrPacket<'p>> {
        if packet.len() >= MutableRtcpRrPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpRrPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableRtcpRrPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableRtcpRrPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableRtcpRrPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableRtcpRrPacket<'static>> {
        if packet.len() >= MutableRtcpRrPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpRrPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableRtcpRrPacket to a RtcpRrPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpRrPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpRrPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableRtcpRrPacket to a RtcpRrPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpRrPacket<'a> {
        RtcpRrPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        4
    }
    /// The size (in bytes) of a RtcpRr instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpRr) -> usize {
        4 + _packet.payload.len()
    }
    /// Populates a RtcpRrPacket using a RtcpRr structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &RtcpRr) {
        let _self = self;
        _self.set_ssrc(packet.ssrc);
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
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 4;
        unsafe {
            copy_nonoverlapping(
                vals[..].as_ptr(),
                _self.packet[current_offset..].as_mut_ptr(),
                vals.len(),
            )
        }
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for RtcpRrPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableRtcpRrPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableRtcpRrPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
        &mut self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 4;
        if _self.packet.len() <= start {
            return &mut [];
        }
        &mut _self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for MutableRtcpRrPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for RtcpRrPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `RtcpRrPacket`s
pub struct RtcpRrIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for RtcpRrIterable<'a> {
    type Item = RtcpRrPacket<'a>;
    fn next(&mut self) -> Option<RtcpRrPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = RtcpRrPacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for RtcpRrPacket<'p> {
    type T = RtcpRr;
    #[inline]
    fn from_packet(&self) -> RtcpRr {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpRr {
            ssrc: _self.get_ssrc(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableRtcpRrPacket<'p> {
    type T = RtcpRr;
    #[inline]
    fn from_packet(&self) -> RtcpRr {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpRr {
            ssrc: _self.get_ssrc(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for RtcpRrPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt, "RtcpRrPacket {{ ssrc : {:?},  }}", _self.get_ssrc())
    }
}
impl<'p> ::std::fmt::Debug for MutableRtcpRrPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "MutableRtcpRrPacket {{ ssrc : {:?},  }}",
            _self.get_ssrc()
        )
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct RtcpRr {
    pub ssrc: u32be,
    pub payload: Vec<u8>,
}
