use std::net::Ipv4Addr;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub listen_addr: Ipv4Addr,
    pub listen_port: u16,
}
