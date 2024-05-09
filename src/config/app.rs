use serde_with::DisplayFromStr;
use std::{net::SocketAddr, str::FromStr};

use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub path: String,
    pub backends: Vec<SocketAddr>,

    #[serde(rename = "lb")]
    #[serde_as(as = "DisplayFromStr")]
    pub load_balancer: LoadBalancerType,
}

#[derive(Debug, Deserialize)]
pub enum LoadBalancerType {
    Random,
}

impl FromStr for LoadBalancerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "random" => Ok(Self::Random),
            unknown => Err(format!("Unknown load balancer type: '{}'", unknown)),
        }
    }
}
