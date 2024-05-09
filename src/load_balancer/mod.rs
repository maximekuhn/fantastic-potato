use std::net::SocketAddr;

pub mod random;

pub trait LoadBalancer: Send {
    fn choose_one(&mut self) -> SocketAddr;
}
