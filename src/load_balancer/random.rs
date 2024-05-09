use std::net::SocketAddr;

use rand::seq::SliceRandom;

use super::LoadBalancer;

pub struct RandomLoadBalancer {
    backends: Vec<SocketAddr>,
}

impl RandomLoadBalancer {
    pub fn new(backends: Vec<SocketAddr>) -> Self {
        Self { backends }
    }
}

impl LoadBalancer for RandomLoadBalancer {
    fn choose_one(&mut self) -> SocketAddr {
        *self
            .backends
            .choose(&mut rand::thread_rng())
            .expect("no backends found")
    }
}
