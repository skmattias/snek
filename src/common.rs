use std::net::{IpAddr, Ipv4Addr, SocketAddr};
pub static ADDR: &'static SocketAddr = 
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);