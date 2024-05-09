use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub path: String,
    pub backends: Vec<SocketAddr>,
}
