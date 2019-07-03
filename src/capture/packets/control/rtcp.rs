use pnet_macros_support::types::*;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct RtcpPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableRtcpPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> RtcpPacket<'a> {
    /// Constructs a new RtcpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<RtcpPacket<'p>> {
        if packet.len() >= RtcpPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new RtcpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the RtcpPacket will
    /// own its own data and the underlying buffer will be dropped when the RtcpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<RtcpPacket<'static>> {
        if packet.len() >= RtcpPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a RtcpPacket to a RtcpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a RtcpPacket to a RtcpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpPacket<'a> {
        RtcpPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        3
    }
    /// The size (in bytes) of a Rtcp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Rtcp) -> usize {
        3 + _packet.payload.len()
    }
    /// Get the version field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_version(&self) -> u2 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u2) & 192) >> 6
    }
    /// Get the padding field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_padding(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 32) >> 5
    }
    /// Get the counter_or_subtype field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_counter_or_subtype(&self) -> u5 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u5) & 31)
    }
    /// Get the packet_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_packet_type(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Get the length field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> u8 {
        let _self = self;
        let co = 2;
        (_self.packet[co] as u8)
    }
}
impl<'a> MutableRtcpPacket<'a> {
    /// Constructs a new MutableRtcpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableRtcpPacket<'p>> {
        if packet.len() >= MutableRtcpPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableRtcpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableRtcpPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableRtcpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableRtcpPacket<'static>> {
        if packet.len() >= MutableRtcpPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableRtcpPacket to a RtcpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableRtcpPacket to a RtcpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpPacket<'a> {
        RtcpPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        3
    }
    /// The size (in bytes) of a Rtcp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Rtcp) -> usize {
        3 + _packet.payload.len()
    }
    /// Populates a RtcpPacket using a Rtcp structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Rtcp) {
        let _self = self;
        _self.set_version(packet.version);
        _self.set_padding(packet.padding);
        _self.set_counter_or_subtype(packet.counter_or_subtype);
        _self.set_packet_type(packet.packet_type);
        _self.set_length(packet.length);
        _self.set_payload(&packet.payload);
    }
    /// Get the version field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_version(&self) -> u2 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u2) & 192) >> 6
    }
    /// Get the padding field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_padding(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 32) >> 5
    }
    /// Get the counter_or_subtype field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_counter_or_subtype(&self) -> u5 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u5) & 31)
    }
    /// Get the packet_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_packet_type(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Get the length field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> u8 {
        let _self = self;
        let co = 2;
        (_self.packet[co] as u8)
    }
    /// Set the version field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_version(&mut self, val: u2) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 63) | (((val & 3) << 6) as u8)) as u8;
    }
    /// Set the padding field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_padding(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 223) | (((val & 1) << 5) as u8)) as u8;
    }
    /// Set the counter_or_subtype field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_counter_or_subtype(&mut self, val: u5) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 224) | ((val & 31) as u8)) as u8;
    }
    /// Set the packet_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_packet_type(&mut self, val: u8) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the length field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_length(&mut self, val: u8) {
        let _self = self;
        let co = 2;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 3;
        unsafe {
            copy_nonoverlapping(
                vals[..].as_ptr(),
                _self.packet[current_offset..].as_mut_ptr(),
                vals.len(),
            )
        }
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for RtcpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        3
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableRtcpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        3
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableRtcpPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
        &mut self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 3;
        if _self.packet.len() <= start {
            return &mut [];
        }
        &mut _self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for MutableRtcpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 3;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for RtcpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 3;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `RtcpPacket`s
pub struct RtcpIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for RtcpIterable<'a> {
    type Item = RtcpPacket<'a>;
    fn next(&mut self) -> Option<RtcpPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = RtcpPacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for RtcpPacket<'p> {
    type T = Rtcp;
    #[inline]
    fn from_packet(&self) -> Rtcp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Rtcp {
            version: _self.get_version(),
            padding: _self.get_padding(),
            counter_or_subtype: _self.get_counter_or_subtype(),
            packet_type: _self.get_packet_type(),
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
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableRtcpPacket<'p> {
    type T = Rtcp;
    #[inline]
    fn from_packet(&self) -> Rtcp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Rtcp {
            version: _self.get_version(),
            padding: _self.get_padding(),
            counter_or_subtype: _self.get_counter_or_subtype(),
            packet_type: _self.get_packet_type(),
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
impl<'p> ::std::fmt::Debug for RtcpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "RtcpPacket {{ version : {:?}, padding : {:?}, counter_or_subtype : {:?}, packet_type : {:?}, length : {:?},  }}"
               , _self . get_version (  ) , _self . get_padding (  ) , _self .
               get_counter_or_subtype (  ) , _self . get_packet_type (  ) ,
               _self . get_length (  ))
    }
}
impl<'p> ::std::fmt::Debug for MutableRtcpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableRtcpPacket {{ version : {:?}, padding : {:?}, counter_or_subtype : {:?}, packet_type : {:?}, length : {:?},  }}"
               , _self . get_version (  ) , _self . get_padding (  ) , _self .
               get_counter_or_subtype (  ) , _self . get_packet_type (  ) ,
               _self . get_length (  ))
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Rtcp {
    pub version: u2,
    pub padding: u1,
    pub counter_or_subtype: u5,
    pub packet_type: u8,
    pub length: u8,
    pub payload: Vec<u8>,
}
