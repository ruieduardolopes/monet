use pnet_macros_support::types::*;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct H264NalPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableH264NalPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> H264NalPacket<'a> {
    /// Constructs a new H264NalPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<H264NalPacket<'p>> {
        if packet.len() >= H264NalPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(H264NalPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new H264NalPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the H264NalPacket will
    /// own its own data and the underlying buffer will be dropped when the H264NalPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<H264NalPacket<'static>> {
        if packet.len() >= H264NalPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(H264NalPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a H264NalPacket to a H264NalPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> H264NalPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        H264NalPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a H264NalPacket to a H264NalPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> H264NalPacket<'a> {
        H264NalPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        1
    }
    /// The size (in bytes) of a H264Nal instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &H264Nal) -> usize {
        1 + _packet.payload.len()
    }
    /// Get the forbidden_zero_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_forbidden_zero_bit(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the nal_ref_idc field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_nal_ref_idc(&self) -> u2 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u2) & 96) >> 5
    }
    /// Get the packet_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_packet_type(&self) -> u5 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u5) & 31)
    }
}
impl<'a> MutableH264NalPacket<'a> {
    /// Constructs a new MutableH264NalPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableH264NalPacket<'p>> {
        if packet.len() >= MutableH264NalPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableH264NalPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableH264NalPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableH264NalPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableH264NalPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableH264NalPacket<'static>> {
        if packet.len() >= MutableH264NalPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableH264NalPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableH264NalPacket to a H264NalPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> H264NalPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        H264NalPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableH264NalPacket to a H264NalPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> H264NalPacket<'a> {
        H264NalPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        1
    }
    /// The size (in bytes) of a H264Nal instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &H264Nal) -> usize {
        1 + _packet.payload.len()
    }
    /// Populates a H264NalPacket using a H264Nal structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &H264Nal) {
        let _self = self;
        _self.set_forbidden_zero_bit(packet.forbidden_zero_bit);
        _self.set_nal_ref_idc(packet.nal_ref_idc);
        _self.set_packet_type(packet.packet_type);
        _self.set_payload(&packet.payload);
    }
    /// Get the forbidden_zero_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_forbidden_zero_bit(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the nal_ref_idc field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_nal_ref_idc(&self) -> u2 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u2) & 96) >> 5
    }
    /// Get the packet_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_packet_type(&self) -> u5 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u5) & 31)
    }
    /// Set the forbidden_zero_bit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_forbidden_zero_bit(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 127) | (((val & 1) << 7) as u8)) as u8;
    }
    /// Set the nal_ref_idc field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_nal_ref_idc(&mut self, val: u2) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 159) | (((val & 3) << 5) as u8)) as u8;
    }
    /// Set the packet_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_packet_type(&mut self, val: u5) {
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
impl<'a> ::pnet_macros_support::packet::PacketSize for H264NalPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableH264NalPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableH264NalPacket<'a> {
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
impl<'a> ::pnet_macros_support::packet::Packet for MutableH264NalPacket<'a> {
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
impl<'a> ::pnet_macros_support::packet::Packet for H264NalPacket<'a> {
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
/// Used to iterate over a slice of `H264NalPacket`s
pub struct H264NalIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for H264NalIterable<'a> {
    type Item = H264NalPacket<'a>;
    fn next(&mut self) -> Option<H264NalPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = H264NalPacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for H264NalPacket<'p> {
    type T = H264Nal;
    #[inline]
    fn from_packet(&self) -> H264Nal {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        H264Nal {
            forbidden_zero_bit: _self.get_forbidden_zero_bit(),
            nal_ref_idc: _self.get_nal_ref_idc(),
            packet_type: _self.get_packet_type(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableH264NalPacket<'p> {
    type T = H264Nal;
    #[inline]
    fn from_packet(&self) -> H264Nal {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        H264Nal {
            forbidden_zero_bit: _self.get_forbidden_zero_bit(),
            nal_ref_idc: _self.get_nal_ref_idc(),
            packet_type: _self.get_packet_type(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for H264NalPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "H264NalPacket {{ forbidden_zero_bit : {:?}, nal_ref_idc : {:?}, packet_type : {:?},  }}"
               , _self . get_forbidden_zero_bit (  ) , _self . get_nal_ref_idc
               (  ) , _self . get_packet_type (  ))
    }
}
impl<'p> ::std::fmt::Debug for MutableH264NalPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableH264NalPacket {{ forbidden_zero_bit : {:?}, nal_ref_idc : {:?}, packet_type : {:?},  }}"
               , _self . get_forbidden_zero_bit (  ) , _self . get_nal_ref_idc
               (  ) , _self . get_packet_type (  ))
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct H264Nal {
    pub forbidden_zero_bit: u1,
    pub nal_ref_idc: u2,
    pub packet_type: u5,
    pub payload: Vec<u8>,
}
