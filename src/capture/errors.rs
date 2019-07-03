#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum CaptureError {
    #[fail(display = "An Ethernet payload failed to be recognized.")]
    UnrecognizableEthernetPayload,
    #[fail(display = "A packet was received, but wrongly made")]
    UndefinedPacket,
    #[fail(display = "A datalink (layer 2) packet failed to be recognized.")]
    UnrecognizableDatalinkPacket,
    #[fail(display = "An IPv4 packet failed to be recognized.")]
    UnrecognizableIPv4Packet,
    #[fail(display = "An IPv6 packet failed to be recognized.")]
    UnrecognizableIPv6Packet,
    #[fail(display = "A transport header failed to be recognized.")]
    UnrecognizableTransportPacket,
    #[fail(display = "A TCP packet seems to be malformed.")]
    MalformedTCPPacket,
    #[fail(display = "A UDP packet seems to be malformed.")]
    MalformedUDPPacket,
    #[fail(display = "Failed RTP packet test on UDP payload.")]
    NotAnRTPPacket,
    #[fail(display = "Failed RTCP packet test on UDP payload.")]
    NotAnRTCPPacket,
    #[fail(display = "Irrelevant RTCP packet on UDP payload.")]
    IrrelevantRTCPPacket,
    #[fail(display = "An RTP packet failed to be recognized.")]
    UnrecognizableRTPPayload,
    #[fail(display = "An RTP's H.264 packet failed to be recognized.")]
    UnrecognizableRTPH264PacketType,
    #[fail(display = "A RTP's H.264 packet seems to be malformed.")]
    MalformedRTPH264Packet,
    #[fail(display = "An RTCP's Sender Report packet failed to be recognized.")]
    MalformedRTCPSRPacket,
    #[fail(display = "An RTP's H.264 Type-A Fragment Unit packet failed to be recognized.")]
    UnrecognizableRTPH264FuAUnitType,
    #[fail(display = "A RTP's H.264 Type-A Fragment Unit packet seems to be malformed.")]
    MalformedRTPH264FuAPacket,
    #[fail(display = "An RTP's H.264 Type-A Fragment Unit Slice failed to be recognized.")]
    UnrecognizableRTPH264SliceType,
    #[fail(display = "A RTP's H.264 Type-A Fragment Unit Slice packet seems to be malformed.")]
    MalformedRTPH264SlicePacket,
    #[fail(display = "The given interface does not exist.")]
    InterfaceNotFound,
    #[fail(
    display = "The current execution does not have enough permissions to create an rx or tx channel."
    )]
    NotEnoughPermissionsToCreateChannel,
}
