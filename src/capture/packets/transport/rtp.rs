use pnet_macros_support::types::*;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct RtpPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableRtpPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> RtpPacket<'a> {
    /// Constructs a new RtpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<RtpPacket<'p>> {
        if packet.len() >= RtpPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtpPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new RtpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the RtpPacket will
    /// own its own data and the underlying buffer will be dropped when the RtpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<RtpPacket<'static>> {
        if packet.len() >= RtpPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtpPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a RtpPacket to a RtpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtpPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtpPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a RtpPacket to a RtpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtpPacket<'a> {
        RtpPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        12
    }
    /// The size (in bytes) of a Rtp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Rtp) -> usize {
        12 + _packet.csrc.len() + _packet.payload.len()
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
    /// Get the extension field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_extension(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 16) >> 4
    }
    /// Get the csrc_count field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_csrc_count(&self) -> u4 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u4) & 15)
    }
    /// Get the marker field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_marker(&self) -> u1 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the payload_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_payload_type(&self) -> u7 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u7) & 127)
    }
    /// Get the sequence_number field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence_number(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = (_self.packet[co + 1] as u16be) as u16be;
        b0 | b1
    }
    /// Get the timestamp field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_timestamp(&self) -> u32be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the ssrc field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ssrc(&self) -> u32be {
        let _self = self;
        let co = 8;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the raw &[u8] value of the csrc field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_csrc_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 12;
        let end = min(
            current_offset + (_self.get_csrc_count() as usize),
            _self.packet.len(),
        );
        &_self.packet[current_offset..end]
    }
    /// Get the value of the csrc field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_csrc(&self) -> Vec<u32be> {
        use std::cmp::min;
        let _self = self;
        let current_offset = 12;
        let pkt_len = self.packet.len();
        let end = min(current_offset + (_self.get_csrc_count() as usize), pkt_len);
        let packet = &_self.packet[current_offset..end];
        let mut vec: Vec<u32be> = Vec::with_capacity(packet.len());
        let mut co = 0;
        for _ in 0..vec.capacity() {
            vec.push({
                let b0 = ((packet[co + 0] as u32be) << 24) as u32be;
                let b1 = ((packet[co + 1] as u32be) << 16) as u32be;
                let b2 = ((packet[co + 2] as u32be) << 8) as u32be;
                let b3 = (packet[co + 3] as u32be) as u32be;
                b0 | b1 | b2 | b3
            });
            co += 4;
        }
        vec
    }
}
impl<'a> MutableRtpPacket<'a> {
    /// Constructs a new MutableRtpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableRtpPacket<'p>> {
        if packet.len() >= MutableRtpPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtpPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableRtpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableRtpPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableRtpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableRtpPacket<'static>> {
        if packet.len() >= MutableRtpPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtpPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableRtpPacket to a RtpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtpPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtpPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableRtpPacket to a RtpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtpPacket<'a> {
        RtpPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        12
    }
    /// The size (in bytes) of a Rtp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Rtp) -> usize {
        12 + _packet.csrc.len() + _packet.payload.len()
    }
    /// Populates a RtpPacket using a Rtp structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Rtp) {
        let _self = self;
        _self.set_version(packet.version);
        _self.set_padding(packet.padding);
        _self.set_extension(packet.extension);
        _self.set_csrc_count(packet.csrc_count);
        _self.set_marker(packet.marker);
        _self.set_payload_type(packet.payload_type);
        _self.set_sequence_number(packet.sequence_number);
        _self.set_timestamp(packet.timestamp);
        _self.set_ssrc(packet.ssrc);
        _self.set_csrc(&packet.csrc);
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
    /// Get the extension field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_extension(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 16) >> 4
    }
    /// Get the csrc_count field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_csrc_count(&self) -> u4 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u4) & 15)
    }
    /// Get the marker field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_marker(&self) -> u1 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the payload_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_payload_type(&self) -> u7 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u7) & 127)
    }
    /// Get the sequence_number field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence_number(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = (_self.packet[co + 1] as u16be) as u16be;
        b0 | b1
    }
    /// Get the timestamp field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_timestamp(&self) -> u32be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the ssrc field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ssrc(&self) -> u32be {
        let _self = self;
        let co = 8;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the raw &[u8] value of the csrc field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_csrc_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 12;
        let end = min(
            current_offset + (_self.get_csrc_count() as usize),
            _self.packet.len(),
        );
        &_self.packet[current_offset..end]
    }
    /// Get the value of the csrc field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_csrc(&self) -> Vec<u32be> {
        use std::cmp::min;
        let _self = self;
        let current_offset = 12;
        let pkt_len = self.packet.len();
        let end = min(current_offset + (_self.get_csrc_count() as usize), pkt_len);
        let packet = &_self.packet[current_offset..end];
        let mut vec: Vec<u32be> = Vec::with_capacity(packet.len());
        let mut co = 0;
        for _ in 0..vec.capacity() {
            vec.push({
                let b0 = ((packet[co + 0] as u32be) << 24) as u32be;
                let b1 = ((packet[co + 1] as u32be) << 16) as u32be;
                let b2 = ((packet[co + 2] as u32be) << 8) as u32be;
                let b3 = (packet[co + 3] as u32be) as u32be;
                b0 | b1 | b2 | b3
            });
            co += 4;
        }
        vec
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
    /// Set the extension field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_extension(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 239) | (((val & 1) << 4) as u8)) as u8;
    }
    /// Set the csrc_count field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_csrc_count(&mut self, val: u4) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 240) | ((val & 15) as u8)) as u8;
    }
    /// Set the marker field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_marker(&mut self, val: u1) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 127) | (((val & 1) << 7) as u8)) as u8;
    }
    /// Set the payload_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload_type(&mut self, val: u7) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] = ((_self.packet[co + 0] & 128) | ((val & 127) as u8)) as u8;
    }
    /// Set the sequence_number field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_sequence_number(&mut self, val: u16be) {
        let _self = self;
        let co = 2;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the timestamp field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_timestamp(&mut self, val: u32be) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the ssrc field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_ssrc(&mut self, val: u32be) {
        let _self = self;
        let co = 8;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Get the raw &mut [u8] value of the csrc field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_csrc_raw_mut(&mut self) -> &mut [u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 12;
        let end = min(
            current_offset + (_self.get_csrc_count() as usize),
            _self.packet.len(),
        );
        &mut _self.packet[current_offset..end]
    }
    /// Set the value of the csrc field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_csrc(&mut self, vals: &[u32be]) {
        #[allow(unused_imports)]
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 12;
        let len = _self.get_csrc_count() as usize;
        assert!(vals.len() <= len);
        let mut co = current_offset;
        for i in 0..vals.len() {
            let val = vals[i];
            _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
            _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
            _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 3] = (val) as u8;
            co += 4;
        }
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 12 + (_self.get_csrc_count() as usize);
        unsafe {
            copy_nonoverlapping(
                vals[..].as_ptr(),
                _self.packet[current_offset..].as_mut_ptr(),
                vals.len(),
            )
        }
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for RtpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        12 + (_self.get_csrc_count() as usize)
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableRtpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        12 + (_self.get_csrc_count() as usize)
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableRtpPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
        &mut self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 12 + (_self.get_csrc_count() as usize);
        if _self.packet.len() <= start {
            return &mut [];
        }
        &mut _self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for MutableRtpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 12 + (_self.get_csrc_count() as usize);
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
impl<'a> ::pnet_macros_support::packet::Packet for RtpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] {
        &self.packet[..]
    }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 12 + (_self.get_csrc_count() as usize);
        if _self.packet.len() <= start {
            return &[];
        }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `RtpPacket`s
pub struct RtpIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for RtpIterable<'a> {
    type Item = RtpPacket<'a>;
    fn next(&mut self) -> Option<RtpPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = RtpPacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for RtpPacket<'p> {
    type T = Rtp;
    #[inline]
    fn from_packet(&self) -> Rtp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Rtp {
            version: _self.get_version(),
            padding: _self.get_padding(),
            extension: _self.get_extension(),
            csrc_count: _self.get_csrc_count(),
            marker: _self.get_marker(),
            payload_type: _self.get_payload_type(),
            sequence_number: _self.get_sequence_number(),
            timestamp: _self.get_timestamp(),
            ssrc: _self.get_ssrc(),
            csrc: _self.get_csrc(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableRtpPacket<'p> {
    type T = Rtp;
    #[inline]
    fn from_packet(&self) -> Rtp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Rtp {
            version: _self.get_version(),
            padding: _self.get_padding(),
            extension: _self.get_extension(),
            csrc_count: _self.get_csrc_count(),
            marker: _self.get_marker(),
            payload_type: _self.get_payload_type(),
            sequence_number: _self.get_sequence_number(),
            timestamp: _self.get_timestamp(),
            ssrc: _self.get_ssrc(),
            csrc: _self.get_csrc(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for RtpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "RtpPacket {{ version : {:?}, padding : {:?}, extension : {:?}, csrc_count : {:?}, marker : {:?}, payload_type : {:?}, sequence_number : {:?}, timestamp : {:?}, ssrc : {:?}, csrc : {:?},  }}"
               , _self . get_version (  ) , _self . get_padding (  ) , _self .
               get_extension (  ) , _self . get_csrc_count (  ) , _self .
               get_marker (  ) , _self . get_payload_type (  ) , _self .
               get_sequence_number (  ) , _self . get_timestamp (  ) , _self .
               get_ssrc (  ) , _self . get_csrc (  ))
    }
}
impl<'p> ::std::fmt::Debug for MutableRtpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableRtpPacket {{ version : {:?}, padding : {:?}, extension : {:?}, csrc_count : {:?}, marker : {:?}, payload_type : {:?}, sequence_number : {:?}, timestamp : {:?}, ssrc : {:?}, csrc : {:?},  }}"
               , _self . get_version (  ) , _self . get_padding (  ) , _self .
               get_extension (  ) , _self . get_csrc_count (  ) , _self .
               get_marker (  ) , _self . get_payload_type (  ) , _self .
               get_sequence_number (  ) , _self . get_timestamp (  ) , _self .
               get_ssrc (  ) , _self . get_csrc (  ))
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Rtp {
    pub version: u2,
    pub padding: u1,
    pub extension: u1,
    pub csrc_count: u4,
    pub marker: u1,
    pub payload_type: u7,
    pub sequence_number: u16be,
    pub timestamp: u32be,
    pub ssrc: u32be,
    pub csrc: Vec<u32be>,
    pub payload: Vec<u8>,
}
