use pnet_macros_support::types::*;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct RtcpSdesItemPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableRtcpSdesItemPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> RtcpSdesItemPacket<'a> {
    /// Constructs a new RtcpSdesItemPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<RtcpSdesItemPacket<'p>> {
        if packet.len() >= RtcpSdesItemPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpSdesItemPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new RtcpSdesItemPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the RtcpSdesItemPacket will
    /// own its own data and the underlying buffer will be dropped when the RtcpSdesItemPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<RtcpSdesItemPacket<'static>> {
        if packet.len() >= RtcpSdesItemPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpSdesItemPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a RtcpSdesItemPacket to a RtcpSdesItemPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpSdesItemPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpSdesItemPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a RtcpSdesItemPacket to a RtcpSdesItemPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpSdesItemPacket<'a> {
        RtcpSdesItemPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        2
    }
    /// The size (in bytes) of a RtcpSdesItem instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpSdesItem) -> usize {
        2 + _packet.payload.len()
    }
    /// Get the item_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_item_type(&self) -> u8 {
        let _self = self;
        let co = 0;
        (_self.packet[co] as u8)
    }
    /// Get the length field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
}
impl<'a> MutableRtcpSdesItemPacket<'a> {
    /// Constructs a new MutableRtcpSdesItemPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableRtcpSdesItemPacket<'p>> {
        if packet.len() >= MutableRtcpSdesItemPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpSdesItemPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableRtcpSdesItemPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableRtcpSdesItemPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableRtcpSdesItemPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableRtcpSdesItemPacket<'static>> {
        if packet.len() >= MutableRtcpSdesItemPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpSdesItemPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableRtcpSdesItemPacket to a RtcpSdesItemPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpSdesItemPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpSdesItemPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableRtcpSdesItemPacket to a RtcpSdesItemPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpSdesItemPacket<'a> {
        RtcpSdesItemPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        2
    }
    /// The size (in bytes) of a RtcpSdesItem instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpSdesItem) -> usize {
        2 + _packet.payload.len()
    }
    /// Populates a RtcpSdesItemPacket using a RtcpSdesItem structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &RtcpSdesItem) {
        let _self = self;
        _self.set_item_type(packet.item_type);
        _self.set_length(packet.length);
        _self.set_payload(&packet.payload);
    }
    /// Get the item_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_item_type(&self) -> u8 {
        let _self = self;
        let co = 0;
        (_self.packet[co] as u8)
    }
    /// Get the length field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Set the item_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_item_type(&mut self, val: u8) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the length field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_length(&mut self, val: u8) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 2;
        unsafe {
            copy_nonoverlapping(
                vals[..].as_ptr(),
                _self.packet[current_offset..].as_mut_ptr(),
                vals.len(),
            )
        }
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for RtcpSdesItemPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        2
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableRtcpSdesItemPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        2
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableRtcpSdesItemPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
        &mut self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 2;
        if _self.packet.len() <= start {
            return &mut [];
        }
        &mut _self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for MutableRtcpSdesItemPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 2;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for RtcpSdesItemPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 2;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `RtcpSdesItemPacket`s
pub struct RtcpSdesItemIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for RtcpSdesItemIterable<'a> {
    type Item = RtcpSdesItemPacket<'a>;
    fn next(&mut self) -> Option<RtcpSdesItemPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = RtcpSdesItemPacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for RtcpSdesItemPacket<'p> {
    type T = RtcpSdesItem;
    #[inline]
    fn from_packet(&self) -> RtcpSdesItem {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpSdesItem {
            item_type: _self.get_item_type(),
            length: _self.get_length(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableRtcpSdesItemPacket<'p> {
    type T = RtcpSdesItem;
    #[inline]
    fn from_packet(&self) -> RtcpSdesItem {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpSdesItem {
            item_type: _self.get_item_type(),
            length: _self.get_length(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for RtcpSdesItemPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "RtcpSdesItemPacket {{ item_type : {:?}, length : {:?},  }}",
            _self.get_item_type(),
            _self.get_length()
        )
    }
}
impl<'p> ::std::fmt::Debug for MutableRtcpSdesItemPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "MutableRtcpSdesItemPacket {{ item_type : {:?}, length : {:?},  }}",
            _self.get_item_type(),
            _self.get_length()
        )
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct RtcpSdesItem {
    pub item_type: u8,
    pub length: u8,
    pub payload: Vec<u8>,
}
