use std::io::Error;
use crate::structs::server::Server;
use std::net::Ipv4Addr;

pub fn init() -> Result<(), Error> {
    let server: Server = Server::new(Ipv4Addr::new(127,0,0,1), 5225);

}