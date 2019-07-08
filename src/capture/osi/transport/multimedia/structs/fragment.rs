use std::net::Ipv4Addr;

#[derive(Deserialize, Serialize)]
pub struct Fragment {
    pub ssrc: u32,
    pub dest_address: Ipv4Addr,
    pub stream_port: u16,
    pub timestamp: u32,
}
