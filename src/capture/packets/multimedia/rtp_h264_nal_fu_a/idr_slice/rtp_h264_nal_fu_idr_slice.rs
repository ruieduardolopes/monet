use pnet_macros_support::types::*;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct H264NalFuANonIDRSlicePacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableH264NalFuANonIDRSlicePacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> H264NalFuANonIDRSlicePacket<'a> {
    /// Constructs a new H264NalFuANonIDRSlicePacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<H264NalFuANonIDRSlicePacket<'p>> {
        if packet.len() >= H264NalFuANonIDRSlicePacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(H264NalFuANonIDRSlicePacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new H264NalFuANonIDRSlicePacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the H264NalFuANonIDRSlicePacket will
    /// own its own data and the underlying buffer will be dropped when the H264NalFuANonIDRSlicePacket is.
    pub fn owned(packet: Vec<u8>) -> Option<H264NalFuANonIDRSlicePacket<'static>> {
        if packet.len() >= H264NalFuANonIDRSlicePacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(H264NalFuANonIDRSlicePacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a H264NalFuANonIDRSlicePacket to a H264NalFuANonIDRSlicePacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> H264NalFuANonIDRSlicePacket<'p> {
        use pnet_macros_support::packet::PacketData;
        H264NalFuANonIDRSlicePacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a H264NalFuANonIDRSlicePacket to a H264NalFuANonIDRSlicePacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> H264NalFuANonIDRSlicePacket<'a> {
        H264NalFuANonIDRSlicePacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        1
    }
    /// The size (in bytes) of a H264NalFuANonIDRSlice instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &H264NalFuANonIDRSlice) -> usize {
        1 + _packet.payload.len()
    }
    /// Get the first_mb_in_slice field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_first_mb_in_slice(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the slice_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_slice_type(&self) -> u7 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u7) & 127)
    }
}
impl<'a> MutableH264NalFuANonIDRSlicePacket<'a> {
    /// Constructs a new MutableH264NalFuANonIDRSlicePacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableH264NalFuANonIDRSlicePacket<'p>> {
        if packet.len() >= MutableH264NalFuANonIDRSlicePacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableH264NalFuANonIDRSlicePacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableH264NalFuANonIDRSlicePacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableH264NalFuANonIDRSlicePacket will
    /// own its own data and the underlying buffer will be dropped when the MutableH264NalFuANonIDRSlicePacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableH264NalFuANonIDRSlicePacket<'static>> {
        if packet.len() >= MutableH264NalFuANonIDRSlicePacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableH264NalFuANonIDRSlicePacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableH264NalFuANonIDRSlicePacket to a H264NalFuANonIDRSlicePacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> H264NalFuANonIDRSlicePacket<'p> {
        use pnet_macros_support::packet::PacketData;
        H264NalFuANonIDRSlicePacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableH264NalFuANonIDRSlicePacket to a H264NalFuANonIDRSlicePacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> H264NalFuANonIDRSlicePacket<'a> {
        H264NalFuANonIDRSlicePacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        1
    }
    /// The size (in bytes) of a H264NalFuANonIDRSlice instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &H264NalFuANonIDRSlice) -> usize {
        1 + _packet.payload.len()
    }
    /// Populates a H264NalFuANonIDRSlicePacket using a H264NalFuANonIDRSlice structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &H264NalFuANonIDRSlice) {
        let _self = self;
        _self.set_first_mb_in_slice(packet.first_mb_in_slice);
        _self.set_slice_type(packet.slice_type);
        _self.set_payload(&packet.payload);
    }
    /// Get the first_mb_in_slice field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_first_mb_in_slice(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the slice_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_slice_type(&self) -> u7 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u7) & 127)
    }
    /// Set the first_mb_in_slice field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_first_mb_in_slice(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 127) | (((val & 1) << 7) as u8)) as u8;
    }
    /// Set the slice_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_slice_type(&mut self, val: u7) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 128) | ((val & 127) as u8)) as u8;
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
impl<'a> ::pnet_macros_support::packet::PacketSize for H264NalFuANonIDRSlicePacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableH264NalFuANonIDRSlicePacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableH264NalFuANonIDRSlicePacket<'a> {
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
impl<'a> ::pnet_macros_support::packet::Packet for MutableH264NalFuANonIDRSlicePacket<'a> {
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
impl<'a> ::pnet_macros_support::packet::Packet for H264NalFuANonIDRSlicePacket<'a> {
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
/// Used to iterate over a slice of `H264NalFuANonIDRSlicePacket`s
pub struct H264NalFuANonIDRSliceIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for H264NalFuANonIDRSliceIterable<'a> {
    type Item = H264NalFuANonIDRSlicePacket<'a>;
    fn next(&mut self) -> Option<H264NalFuANonIDRSlicePacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = H264NalFuANonIDRSlicePacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for H264NalFuANonIDRSlicePacket<'p> {
    type T = H264NalFuANonIDRSlice;
    #[inline]
    fn from_packet(&self) -> H264NalFuANonIDRSlice {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        H264NalFuANonIDRSlice {
            first_mb_in_slice: _self.get_first_mb_in_slice(),
            slice_type: _self.get_slice_type(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableH264NalFuANonIDRSlicePacket<'p> {
    type T = H264NalFuANonIDRSlice;
    #[inline]
    fn from_packet(&self) -> H264NalFuANonIDRSlice {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        H264NalFuANonIDRSlice {
            first_mb_in_slice: _self.get_first_mb_in_slice(),
            slice_type: _self.get_slice_type(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for H264NalFuANonIDRSlicePacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(
            fmt,
            "H264NalFuANonIDRSlicePacket {{ first_mb_in_slice : {:?}, slice_type : {:?},  }}",
            _self.get_first_mb_in_slice(),
            _self.get_slice_type()
        )
    }
}
impl<'p> ::std::fmt::Debug for MutableH264NalFuANonIDRSlicePacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableH264NalFuANonIDRSlicePacket {{ first_mb_in_slice : {:?}, slice_type : {:?},  }}"
               , _self . get_first_mb_in_slice (  ) , _self . get_slice_type (
                ))
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct H264NalFuANonIDRSlice {
    pub first_mb_in_slice: u1,
    pub slice_type: u7,
    pub payload: Vec<u8>,
}
