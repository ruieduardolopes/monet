use std::net::{Ipv4Addr, UdpSocket};

pub(crate) struct Server {
    address: Ipv4Addr,
    port: u16,
    socket: UdpSocket,
}

impl Server {
    pub fn new(address: Ipv4Addr, port: u16) -> Self {
        Server {
            address,
            port,
            socket: UdpSocket::bind(format!("{}:{}", address, port)).unwrap(),
        }
    }

    pub fn accept(&self) {}
}
