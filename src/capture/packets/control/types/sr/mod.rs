use pnet_macros_support::types::*;

pub mod rtcp_sr_report_block;

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct RtcpSrPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableRtcpSrPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl<'a> RtcpSrPacket<'a> {
    /// Constructs a new RtcpSrPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<RtcpSrPacket<'p>> {
        if packet.len() >= RtcpSrPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpSrPacket {
                packet: PacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new RtcpSrPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the RtcpSrPacket will
    /// own its own data and the underlying buffer will be dropped when the RtcpSrPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<RtcpSrPacket<'static>> {
        if packet.len() >= RtcpSrPacket::minimum_packet_size() {
            use pnet_macros_support::packet::PacketData;
            Some(RtcpSrPacket {
                packet: PacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a RtcpSrPacket to a RtcpSrPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpSrPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpSrPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a RtcpSrPacket to a RtcpSrPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpSrPacket<'a> {
        RtcpSrPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        24
    }
    /// The size (in bytes) of a RtcpSr instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpSr) -> usize {
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
    /// Get the ntp_timestamp field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ntp_timestamp(&self) -> u64be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u64be) << 56) as u64be;
        let b1 = ((_self.packet[co + 1] as u64be) << 48) as u64be;
        let b2 = ((_self.packet[co + 2] as u64be) << 40) as u64be;
        let b3 = ((_self.packet[co + 3] as u64be) << 32) as u64be;
        let b4 = ((_self.packet[co + 4] as u64be) << 24) as u64be;
        let b5 = ((_self.packet[co + 5] as u64be) << 16) as u64be;
        let b6 = ((_self.packet[co + 6] as u64be) << 8) as u64be;
        let b7 = (_self.packet[co + 7] as u64be) as u64be;
        b0 | b1 | b2 | b3 | b4 | b5 | b6 | b7
    }
    /// Get the rtp_timestamp field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_rtp_timestamp(&self) -> u32be {
        let _self = self;
        let co = 12;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the senders_packet_count field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_senders_packet_count(&self) -> u32be {
        let _self = self;
        let co = 16;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the senders_octet_count field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_senders_octet_count(&self) -> u32be {
        let _self = self;
        let co = 20;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
}
impl<'a> MutableRtcpSrPacket<'a> {
    /// Constructs a new MutableRtcpSrPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableRtcpSrPacket<'p>> {
        if packet.len() >= MutableRtcpSrPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpSrPacket {
                packet: MutPacketData::Borrowed(packet),
            })
        } else {
            None
        }
    }
    /// Constructs a new MutableRtcpSrPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableRtcpSrPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableRtcpSrPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableRtcpSrPacket<'static>> {
        if packet.len() >= MutableRtcpSrPacket::minimum_packet_size() {
            use pnet_macros_support::packet::MutPacketData;
            Some(MutableRtcpSrPacket {
                packet: MutPacketData::Owned(packet),
            })
        } else {
            None
        }
    }
    /// Maps from a MutableRtcpSrPacket to a RtcpSrPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RtcpSrPacket<'p> {
        use pnet_macros_support::packet::PacketData;
        RtcpSrPacket {
            packet: PacketData::Borrowed(self.packet.as_slice()),
        }
    }
    /// Maps from a MutableRtcpSrPacket to a RtcpSrPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RtcpSrPacket<'a> {
        RtcpSrPacket {
            packet: self.packet.to_immutable(),
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize {
        24
    }
    /// The size (in bytes) of a RtcpSr instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &RtcpSr) -> usize {
        24 + _packet.payload.len()
    }
    /// Populates a RtcpSrPacket using a RtcpSr structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &RtcpSr) {
        let _self = self;
        _self.set_ssrc(packet.ssrc);
        _self.set_ntp_timestamp(packet.ntp_timestamp);
        _self.set_rtp_timestamp(packet.rtp_timestamp);
        _self.set_senders_packet_count(packet.senders_packet_count);
        _self.set_senders_octet_count(packet.senders_octet_count);
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
    /// Get the ntp_timestamp field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ntp_timestamp(&self) -> u64be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u64be) << 56) as u64be;
        let b1 = ((_self.packet[co + 1] as u64be) << 48) as u64be;
        let b2 = ((_self.packet[co + 2] as u64be) << 40) as u64be;
        let b3 = ((_self.packet[co + 3] as u64be) << 32) as u64be;
        let b4 = ((_self.packet[co + 4] as u64be) << 24) as u64be;
        let b5 = ((_self.packet[co + 5] as u64be) << 16) as u64be;
        let b6 = ((_self.packet[co + 6] as u64be) << 8) as u64be;
        let b7 = (_self.packet[co + 7] as u64be) as u64be;
        b0 | b1 | b2 | b3 | b4 | b5 | b6 | b7
    }
    /// Get the rtp_timestamp field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_rtp_timestamp(&self) -> u32be {
        let _self = self;
        let co = 12;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the senders_packet_count field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_senders_packet_count(&self) -> u32be {
        let _self = self;
        let co = 16;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = (_self.packet[co + 3] as u32be) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the senders_octet_count field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_senders_octet_count(&self) -> u32be {
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
    /// Set the ntp_timestamp field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_ntp_timestamp(&mut self, val: u64be) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = ((val & 18374686479671623680) >> 56) as u8;
        _self.packet[co + 1] = ((val & 71776119061217280) >> 48) as u8;
        _self.packet[co + 2] = ((val & 280375465082880) >> 40) as u8;
        _self.packet[co + 3] = ((val & 1095216660480) >> 32) as u8;
        _self.packet[co + 4] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 5] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 6] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 7] = (val) as u8;
    }
    /// Set the rtp_timestamp field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_rtp_timestamp(&mut self, val: u32be) {
        let _self = self;
        let co = 12;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the senders_packet_count field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_senders_packet_count(&mut self, val: u32be) {
        let _self = self;
        let co = 16;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the senders_octet_count field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_senders_octet_count(&mut self, val: u32be) {
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
impl<'a> ::pnet_macros_support::packet::PacketSize for RtcpSrPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        24
    }
}
impl<'a> ::pnet_macros_support::packet::PacketSize for MutableRtcpSrPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        24
    }
}
impl<'a> ::pnet_macros_support::packet::MutablePacket for MutableRtcpSrPacket<'a> {
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
impl<'a> ::pnet_macros_support::packet::Packet for MutableRtcpSrPacket<'a> {
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
impl<'a> ::pnet_macros_support::packet::Packet for RtcpSrPacket<'a> {
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
/// Used to iterate over a slice of `RtcpSrPacket`s
pub struct RtcpSrIterable<'a> {
    buf: &'a [u8],
}
impl<'a> Iterator for RtcpSrIterable<'a> {
    type Item = RtcpSrPacket<'a>;
    fn next(&mut self) -> Option<RtcpSrPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = RtcpSrPacket::new(self.buf) {
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
impl<'p> ::pnet_macros_support::packet::FromPacket for RtcpSrPacket<'p> {
    type T = RtcpSr;
    #[inline]
    fn from_packet(&self) -> RtcpSr {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpSr {
            ssrc: _self.get_ssrc(),
            ntp_timestamp: _self.get_ntp_timestamp(),
            rtp_timestamp: _self.get_rtp_timestamp(),
            senders_packet_count: _self.get_senders_packet_count(),
            senders_octet_count: _self.get_senders_octet_count(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::pnet_macros_support::packet::FromPacket for MutableRtcpSrPacket<'p> {
    type T = RtcpSr;
    #[inline]
    fn from_packet(&self) -> RtcpSr {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        RtcpSr {
            ssrc: _self.get_ssrc(),
            ntp_timestamp: _self.get_ntp_timestamp(),
            rtp_timestamp: _self.get_rtp_timestamp(),
            senders_packet_count: _self.get_senders_packet_count(),
            senders_octet_count: _self.get_senders_octet_count(),
            payload: {
                let payload = self.payload();
                let mut vec = Vec::with_capacity(payload.len());
                vec.extend_from_slice(payload);
                vec
            },
        }
    }
}
impl<'p> ::std::fmt::Debug for RtcpSrPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "RtcpSrPacket {{ ssrc : {:?}, ntp_timestamp : {:?}, rtp_timestamp : {:?}, senders_packet_count : {:?}, senders_octet_count : {:?},  }}"
               , _self . get_ssrc (  ) , _self . get_ntp_timestamp (  ) ,
               _self . get_rtp_timestamp (  ) , _self .
               get_senders_packet_count (  ) , _self . get_senders_octet_count
               (  ))
    }
}
impl<'p> ::std::fmt::Debug for MutableRtcpSrPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableRtcpSrPacket {{ ssrc : {:?}, ntp_timestamp : {:?}, rtp_timestamp : {:?}, senders_packet_count : {:?}, senders_octet_count : {:?},  }}"
               , _self . get_ssrc (  ) , _self . get_ntp_timestamp (  ) ,
               _self . get_rtp_timestamp (  ) , _self .
               get_senders_packet_count (  ) , _self . get_senders_octet_count
               (  ))
    }
}
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct RtcpSr {
    pub ssrc: u32be,
    pub ntp_timestamp: u64be,
    pub rtp_timestamp: u32be,
    pub senders_packet_count: u32be,
    pub senders_octet_count: u32be,
    pub payload: Vec<u8>,
}
