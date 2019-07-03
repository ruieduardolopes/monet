use pnet_macros_support::types::*;

pub mod rtcp_sdes_item;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct RtcpSdesPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableRtcpSdesPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> RtcpSdesPacket<'a> {
    /// Constructs a new RtcpSdesPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<RtcpSdesPacket<'p>> {
        if packet.len() >= RtcpSdesPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpSdesPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new RtcpSdesPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the RtcpSdesPacket will
    /// own its own data and the underlying buffer will be dropped when the RtcpSdesPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<RtcpSdesPacket<'static>> {
        if packet.len() >= RtcpSdesPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpSdesPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a RtcpSdesPacket to a RtcpSdesPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpSdesPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpSdesPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a RtcpSdesPacket to a RtcpSdesPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpSdesPacket<'a> {
        RtcpSdesPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        4
    }
    /// The size (in bytes) of a RtcpSdes instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpSdes) -> usize {
        4 + _packet.payload.len()
    }
    /// Get the identifier field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_identifier(&self) -> u32be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
}
impl<'a> MutableRtcpSdesPacket<'a> {
    /// Constructs a new MutableRtcpSdesPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableRtcpSdesPacket<'p>> {
        if packet.len() >= MutableRtcpSdesPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpSdesPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableRtcpSdesPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableRtcpSdesPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableRtcpSdesPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableRtcpSdesPacket<'static>> {
        if packet.len() >= MutableRtcpSdesPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpSdesPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableRtcpSdesPacket to a RtcpSdesPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpSdesPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpSdesPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableRtcpSdesPacket to a RtcpSdesPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpSdesPacket<'a> {
        RtcpSdesPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        4
    }
    /// The size (in bytes) of a RtcpSdes instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpSdes) -> usize {
        4 + _packet.payload.len()
    }
    /// Populates a RtcpSdesPacket using a RtcpSdes structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &RtcpSdes) {
        let _self = self;
        _self.set_identifier(packet.identifier);
        _self.set_payload(&packet.payload);
    }
    /// Get the identifier field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_identifier(&self) -> u32be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Set the identifier field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_identifier(&mut self, val: u32be) {
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
impl<'a> ::pnet_macros_support::packet::PacketSize for RtcpSdesPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableRtcpSdesPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableRtcpSdesPacket<'a> {
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
impl<'a> ::pnet_macros_support::packet::Packet for MutableRtcpSdesPacket<'a> {
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
impl<'a> ::pnet_macros_support::packet::Packet for RtcpSdesPacket<'a> {
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
/// Used to iterate over a slice of `RtcpSdesPacket`s
pub struct RtcpSdesIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for RtcpSdesIterable<'a> {
    type Item = RtcpSdesPacket<'a>;
    fn next(&mut self) -> Option<RtcpSdesPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = RtcpSdesPacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for RtcpSdesPacket<'p> {
    type T = RtcpSdes;
    #[inline]
    fn from_packet(&self) -> RtcpSdes {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpSdes {
            identifier: _self.get_identifier(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableRtcpSdesPacket<'p> {
    type T = RtcpSdes;
    #[inline]
    fn from_packet(&self) -> RtcpSdes {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpSdes {
            identifier: _self.get_identifier(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for RtcpSdesPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "RtcpSdesPacket {{ identifier : {:?},  }}",
            _self.get_identifier()
        )
    }
}
impl<'p> ::std::fmt::Debug for MutableRtcpSdesPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "MutableRtcpSdesPacket {{ identifier : {:?},  }}",
            _self.get_identifier()
        )
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct RtcpSdes {
    pub identifier: u32be,
    pub payload: Vec<u8>,
}
