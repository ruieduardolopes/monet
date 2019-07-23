use std::net::Ipv4Addr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fragment {
    pub ssrc: u32,
    pub dest_address: Ipv4Addr,
    pub stream_port: u16,
    pub timestamp: u32,
}

impl Fragment {
    pub fn new(ssrc: u32, dest_address: Ipv4Addr, stream_port: u16, timestamp: u32) -> Self {
        Fragment {
            ssrc,
            dest_address,
            stream_port,
            timestamp,
        }
    }
}
