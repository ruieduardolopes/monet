use pnet_macros_support::types::*;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct H264NalFuAPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableH264NalFuAPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> H264NalFuAPacket<'a> {
    /// Constructs a new H264NalFuAPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<H264NalFuAPacket<'p>> {
        if packet.len() >= H264NalFuAPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(H264NalFuAPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new H264NalFuAPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the H264NalFuAPacket will
    /// own its own data and the underlying buffer will be dropped when the H264NalFuAPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<H264NalFuAPacket<'static>> {
        if packet.len() >= H264NalFuAPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(H264NalFuAPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a H264NalFuAPacket to a H264NalFuAPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> H264NalFuAPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        H264NalFuAPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a H264NalFuAPacket to a H264NalFuAPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> H264NalFuAPacket<'a> {
        H264NalFuAPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        1
    }
    /// The size (in bytes) of a H264NalFuA instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &H264NalFuA) -> usize {
        1 + _packet.payload.len()
    }
    /// Get the start_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_start_bit(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the end_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_end_bit(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 64) >> 6
    }
    /// Get the reserved_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_reserved_bit(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 32) >> 5
    }
    /// Get the unit_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_unit_type(&self) -> u5 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u5) & 31)
    }
}
impl<'a> MutableH264NalFuAPacket<'a> {
    /// Constructs a new MutableH264NalFuAPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableH264NalFuAPacket<'p>> {
        if packet.len() >= MutableH264NalFuAPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableH264NalFuAPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableH264NalFuAPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableH264NalFuAPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableH264NalFuAPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableH264NalFuAPacket<'static>> {
        if packet.len() >= MutableH264NalFuAPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableH264NalFuAPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableH264NalFuAPacket to a H264NalFuAPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> H264NalFuAPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        H264NalFuAPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableH264NalFuAPacket to a H264NalFuAPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> H264NalFuAPacket<'a> {
        H264NalFuAPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        1
    }
    /// The size (in bytes) of a H264NalFuA instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &H264NalFuA) -> usize {
        1 + _packet.payload.len()
    }
    /// Populates a H264NalFuAPacket using a H264NalFuA structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &H264NalFuA) {
        let _self = self;
        _self.set_start_bit(packet.start_bit);
        _self.set_end_bit(packet.end_bit);
        _self.set_reserved_bit(packet.reserved_bit);
        _self.set_unit_type(packet.unit_type);
        _self.set_payload(&packet.payload);
    }
    /// Get the start_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_start_bit(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the end_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_end_bit(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 64) >> 6
    }
    /// Get the reserved_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_reserved_bit(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 32) >> 5
    }
    /// Get the unit_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_unit_type(&self) -> u5 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u5) & 31)
    }
    /// Set the start_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_start_bit(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 127) | (((val & 1) << 7) as u8)) as u8;
    }
    /// Set the end_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_end_bit(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 191) | (((val & 1) << 6) as u8)) as u8;
    }
    /// Set the reserved_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_reserved_bit(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 223) | (((val & 1) << 5) as u8)) as u8;
    }
    /// Set the unit_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_unit_type(&mut self, val: u5) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 224) | ((val & 31) as u8)) as u8;
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 1;
        unsafe {
            copy_nonoverlapping(
                vals[..].as_ptr(),
                _self.packet[current_offset..].as_mut_ptr(),
                vals.len(),
            )
        }
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for H264NalFuAPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableH264NalFuAPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableH264NalFuAPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
        &mut self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 1;
        if _self.packet.len() <= start {
            return &mut [];
        }
        &mut _self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for MutableH264NalFuAPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 1;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for H264NalFuAPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 1;
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `H264NalFuAPacket`s
pub struct H264NalFuAIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for H264NalFuAIterable<'a> {
    type Item = H264NalFuAPacket<'a>;
    fn next(&mut self) -> Option<H264NalFuAPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = H264NalFuAPacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for H264NalFuAPacket<'p> {
    type T = H264NalFuA;
    #[inline]
    fn from_packet(&self) -> H264NalFuA {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        H264NalFuA {
            start_bit: _self.get_start_bit(),
            end_bit: _self.get_end_bit(),
            reserved_bit: _self.get_reserved_bit(),
            unit_type: _self.get_unit_type(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableH264NalFuAPacket<'p> {
    type T = H264NalFuA;
    #[inline]
    fn from_packet(&self) -> H264NalFuA {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        H264NalFuA {
            start_bit: _self.get_start_bit(),
            end_bit: _self.get_end_bit(),
            reserved_bit: _self.get_reserved_bit(),
            unit_type: _self.get_unit_type(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for H264NalFuAPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "H264NalFuAPacket {{ start_bit : {:?}, end_bit : {:?}, reserved_bit : {:?}, unit_type : {:?},  }}"
               , _self . get_start_bit (  ) , _self . get_end_bit (  ) , _self
               . get_reserved_bit (  ) , _self . get_unit_type (  ))
    }
}
impl<'p> ::std::fmt::Debug for MutableH264NalFuAPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableH264NalFuAPacket {{ start_bit : {:?}, end_bit : {:?}, reserved_bit : {:?}, unit_type : {:?},  }}"
               , _self . get_start_bit (  ) , _self . get_end_bit (  ) , _self
               . get_reserved_bit (  ) , _self . get_unit_type (  ))
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct H264NalFuA {
    pub start_bit: u1,
    pub end_bit: u1,
    pub reserved_bit: u1,
    pub unit_type: u5,
    pub payload: Vec<u8>,
}
